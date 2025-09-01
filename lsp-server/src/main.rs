use anyhow::Result;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs;
use tokio::process::Command as ProcessCommand;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tokio::io::{stdin, stdout};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use url::Url;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct InitializationOptions {
    ruleset: Option<String>,
    format: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PhpmdSettings {
    ruleset: Option<String>,
    format: Option<String>,
}

#[derive(Debug, Clone)]
struct CompressedDocument {
    compressed_data: Vec<u8>,
    original_size: usize,
    checksum: String,
    compression_ratio: f32,
}

#[derive(Debug, Clone)]
struct CachedResults {
    diagnostics: Vec<Diagnostic>,
    result_id: String,
    generated_at: Instant,
}

#[derive(Debug, Clone)]
struct PhpmdLanguageServer {
    client: Client,
    // Compressed document storage to reduce memory usage
    open_docs: std::sync::Arc<std::sync::RwLock<HashMap<Url, CompressedDocument>>>,
    // Cache PHPMD results to avoid redundant analysis
    results_cache: std::sync::Arc<std::sync::RwLock<HashMap<Url, CachedResults>>>,
    // Memory tracking
    total_memory_usage: std::sync::Arc<AtomicUsize>,
    ruleset: std::sync::Arc<std::sync::RwLock<Option<String>>>,  // None means use PHPMD defaults
    format: std::sync::Arc<std::sync::RwLock<String>>,  // json or xml
    phpmd_path: std::sync::Arc<std::sync::RwLock<Option<String>>>,
    workspace_root: std::sync::Arc<std::sync::RwLock<Option<std::path::PathBuf>>>,
    // Limit concurrent PHPMD processes to prevent system overload
    process_semaphore: std::sync::Arc<Semaphore>,
}

// PHPMD JSON output structures
#[derive(Debug, Deserialize)]
struct PhpmdJsonOutput {
    files: Vec<PhpmdFile>,
}

#[derive(Debug, Deserialize)]
struct PhpmdFile {
    file: String,
    violations: Vec<PhpmdViolation>,
}

#[derive(Debug, Deserialize)]
struct PhpmdViolation {
    #[serde(rename = "beginLine")]
    begin_line: u32,
    #[serde(rename = "endLine")]
    end_line: u32,
    package: Option<String>,
    function: Option<String>,
    class: Option<String>,
    method: Option<String>,
    description: String,
    rule: String,
    #[serde(rename = "ruleSet")]
    rule_set: String,
    priority: u32,
    #[serde(rename = "externalInfoUrl")]
    external_info_url: Option<String>,
}

impl PhpmdLanguageServer {
    fn new(client: Client) -> Self {
        Self {
            client,
            open_docs: std::sync::Arc::new(std::sync::RwLock::new(HashMap::with_capacity(100))),
            results_cache: std::sync::Arc::new(std::sync::RwLock::new(HashMap::with_capacity(100))),
            total_memory_usage: std::sync::Arc::new(AtomicUsize::new(0)),
            ruleset: std::sync::Arc::new(std::sync::RwLock::new(None)),  // Let PHPMD use its defaults
            format: std::sync::Arc::new(std::sync::RwLock::new("json".to_string())),
            phpmd_path: std::sync::Arc::new(std::sync::RwLock::new(None)),
            workspace_root: std::sync::Arc::new(std::sync::RwLock::new(None)),
            // Limit to 4 concurrent PHPMD processes to avoid overwhelming the system
            process_semaphore: std::sync::Arc::new(Semaphore::new(4)),
        }
    }

    fn compress_document(&self, content: &str) -> CompressedDocument {
        let start = Instant::now();
        let original_size = content.len();

        // Use LZ4 for fast compression
        let compressed_data = compress_prepend_size(content.as_bytes());
        let compressed_size = compressed_data.len();
        let compression_ratio = compressed_size as f32 / original_size as f32;

        // Compute checksum for cache invalidation
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let checksum = format!("{:x}", hasher.finalize());

        let elapsed = start.elapsed();
        eprintln!("📦 PHPMD LSP: Compressed in {:.2}ms: {}KB → {}KB ({:.1}% ratio)",
            elapsed.as_secs_f64() * 1000.0,
            original_size / 1024,
            compressed_size / 1024,
            compression_ratio * 100.0
        );

        // Update memory tracking
        self.total_memory_usage.fetch_add(compressed_size, Ordering::Relaxed);

        CompressedDocument {
            compressed_data,
            original_size,
            checksum,
            compression_ratio,
        }
    }

    fn decompress_document(&self, compressed_doc: &CompressedDocument) -> Result<String> {
        let start = Instant::now();
        let decompressed = decompress_size_prepended(&compressed_doc.compressed_data)?;
        let content = String::from_utf8(decompressed)?;
        
        let elapsed = start.elapsed();
        eprintln!("📂 PHPMD LSP: Decompressed in {:.2}ms: {} bytes",
            elapsed.as_secs_f64() * 1000.0,
            compressed_doc.original_size
        );

        Ok(content)
    }

    async fn run_phpmd(&self, uri: &Url) -> Result<Vec<Diagnostic>> {
        // Get the document content from our compressed storage
        let content = {
            let docs = self.open_docs.read().unwrap();
            match docs.get(uri) {
                Some(compressed_doc) => self.decompress_document(compressed_doc)?,
                None => return Ok(vec![]),
            }
        };

        // Check cache first
        let checksum = {
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            format!("{:x}", hasher.finalize())
        };

        // Check if we have cached results for this checksum
        {
            let cache = self.results_cache.read().unwrap();
            if let Some(cached) = cache.get(uri) {
                if cached.result_id == checksum {
                    let age = cached.generated_at.elapsed();
                    if age < Duration::from_secs(5) {
                        eprintln!("📋 PHPMD LSP: Using cached results (age: {:.1}s)", age.as_secs_f64());
                        return Ok(cached.diagnostics.clone());
                    }
                }
            }
        }

        // Acquire permit to limit concurrent processes
        let _permit = self.process_semaphore.acquire().await?;

        // Create a temporary file for PHPMD to analyze
        let temp_file = std::env::temp_dir().join(format!("phpmd_{}.php", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        
        fs::write(&temp_file, &content)?;

        // Find PHPMD executable
        let phpmd_path = self.find_phpmd_executable().await?;
        
        let ruleset = self.ruleset.read().unwrap().clone()
            .unwrap_or_else(|| "cleancode,codesize,controversial,design,naming,unusedcode".to_string());
        
        let format = self.format.read().unwrap().clone();

        eprintln!("🔍 PHPMD LSP: Running analysis with ruleset: {}", ruleset);

        // Run PHPMD with timeout
        let output = timeout(Duration::from_secs(10), async {
            ProcessCommand::new(&phpmd_path)
                .arg(&temp_file)
                .arg(&format)
                .arg(&ruleset)
                .output()
                .await
        }).await??;

        // Clean up temp file
        let _ = fs::remove_file(&temp_file);

        // Parse the output based on format
        let diagnostics = if format == "json" {
            self.parse_json_output(&output.stdout)?
        } else {
            self.parse_xml_output(&output.stdout)?
        };

        // Cache the results
        {
            let mut cache = self.results_cache.write().unwrap();
            cache.insert(uri.clone(), CachedResults {
                diagnostics: diagnostics.clone(),
                result_id: checksum,
                generated_at: Instant::now(),
            });
        }

        Ok(diagnostics)
    }

    fn parse_json_output(&self, output: &[u8]) -> Result<Vec<Diagnostic>> {
        let output_str = String::from_utf8_lossy(output);
        
        // PHPMD outputs to stdout even when there are no violations
        if output_str.trim().is_empty() {
            return Ok(vec![]);
        }

        let phpmd_output: PhpmdJsonOutput = match serde_json::from_str(&output_str) {
            Ok(output) => output,
            Err(e) => {
                eprintln!("⚠️ PHPMD LSP: Failed to parse JSON output: {}", e);
                eprintln!("Output was: {}", output_str);
                return Ok(vec![]);
            }
        };

        let mut diagnostics = Vec::new();

        for file in phpmd_output.files {
            for violation in file.violations {
                let severity = match violation.priority {
                    1 => DiagnosticSeverity::ERROR,
                    2 => DiagnosticSeverity::WARNING,
                    3 => DiagnosticSeverity::WARNING,
                    4 => DiagnosticSeverity::INFORMATION,
                    5 => DiagnosticSeverity::HINT,
                    _ => DiagnosticSeverity::INFORMATION,
                };

                let mut message = format!("{} ({})", violation.description, violation.rule);
                
                if let Some(ref url) = &violation.external_info_url {
                    message.push_str(&format!("\nMore info: {}", url));
                }

                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position {
                            line: violation.begin_line.saturating_sub(1),
                            character: 0,
                        },
                        end: Position {
                            line: violation.end_line.saturating_sub(1),
                            character: 999, // We don't have column info from PHPMD
                        },
                    },
                    severity: Some(severity),
                    code: Some(NumberOrString::String(violation.rule)),
                    source: Some(format!("phpmd:{}", violation.rule_set)),
                    message,
                    related_information: None,
                    tags: None,
                    code_description: violation.external_info_url.map(|url| CodeDescription {
                        href: Url::parse(&url).ok().unwrap_or_else(|| Url::parse("https://phpmd.org").unwrap()),
                    }),
                    data: None,
                };

                diagnostics.push(diagnostic);
            }
        }

        Ok(diagnostics)
    }

    fn parse_xml_output(&self, _output: &[u8]) -> Result<Vec<Diagnostic>> {
        // XML parsing would go here - for now we'll just use JSON
        eprintln!("⚠️ PHPMD LSP: XML parsing not yet implemented, using JSON format");
        Ok(vec![])
    }

    async fn find_phpmd_executable(&self) -> Result<String> {
        // Check if we have a cached path
        if let Some(path) = self.phpmd_path.read().unwrap().as_ref() {
            if fs::metadata(path).is_ok() {
                return Ok(path.clone());
            }
        }

        // Try to find PHPMD in various locations
        let possible_paths = vec![
            // Project local installation
            "vendor/bin/phpmd",
            // Extension bundled PHAR (in the same directory as the LSP server)
            "phpmd.phar",
            "../phpmd.phar",
            "../../phpmd.phar",
            // System-wide installation
            "phpmd",
        ];

        // If we have a workspace root, check project-local paths first
        if let Some(workspace_root) = self.workspace_root.read().unwrap().as_ref() {
            for path in &possible_paths {
                let full_path = workspace_root.join(path);
                if full_path.exists() {
                    let path_str = full_path.to_string_lossy().to_string();
                    eprintln!("✅ PHPMD LSP: Found PHPMD at: {}", path_str);
                    *self.phpmd_path.write().unwrap() = Some(path_str.clone());
                    return Ok(path_str);
                }
            }
        }

        // Try to find in PATH or relative to the LSP server binary
        for path in &possible_paths {
            // Try as-is (for system PATH or relative paths)
            if let Ok(output) = ProcessCommand::new("which")
                .arg(path)
                .output()
                .await
            {
                if output.status.success() {
                    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path_str.is_empty() {
                        eprintln!("✅ PHPMD LSP: Found PHPMD at: {}", path_str);
                        *self.phpmd_path.write().unwrap() = Some(path_str.clone());
                        return Ok(path_str);
                    }
                }
            }

            // Check if it exists as a file relative to current directory
            if fs::metadata(path).is_ok() {
                let full_path = std::env::current_dir()?.join(path);
                let path_str = full_path.to_string_lossy().to_string();
                eprintln!("✅ PHPMD LSP: Found PHPMD at: {}", path_str);
                *self.phpmd_path.write().unwrap() = Some(path_str.clone());
                return Ok(path_str);
            }
        }

        Err(anyhow::anyhow!("PHPMD executable not found. Please install PHPMD or ensure phpmd.phar is in the extension directory"))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for PhpmdLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        eprintln!("🚀 PHPMD LSP: Initializing server...");

        // Store workspace root
        if let Some(root_uri) = params.root_uri {
            if let Ok(path) = root_uri.to_file_path() {
                *self.workspace_root.write().unwrap() = Some(path);
            }
        }

        // Process initialization options
        if let Some(init_options) = params.initialization_options {
            if let Ok(options) = serde_json::from_value::<InitializationOptions>(init_options) {
                if let Some(ruleset) = options.ruleset {
                    eprintln!("📋 PHPMD LSP: Using ruleset from init options: {}", ruleset);
                    *self.ruleset.write().unwrap() = Some(ruleset);
                }
                if let Some(format) = options.format {
                    eprintln!("📋 PHPMD LSP: Using format from init options: {}", format);
                    *self.format.write().unwrap() = format;
                }
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("phpmd".to_string()),
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        eprintln!("✅ PHPMD LSP: Server initialized successfully");
    }

    async fn shutdown(&self) -> LspResult<()> {
        eprintln!("👋 PHPMD LSP: Shutting down server...");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        
        // Only process PHP files
        if !uri.path().ends_with(".php") {
            return;
        }

        eprintln!("📂 PHPMD LSP: Document opened: {}", uri);

        // Compress and store the document
        let compressed = self.compress_document(&params.text_document.text);
        self.open_docs.write().unwrap().insert(uri.clone(), compressed);

        // Run PHPMD and publish diagnostics
        if let Ok(diagnostics) = self.run_phpmd(&uri).await {
            eprintln!("📊 PHPMD LSP: Found {} issues", diagnostics.len());
            self.client
                .publish_diagnostics(uri, diagnostics, Some(params.text_document.version))
                .await;
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();

        // Only process PHP files
        if !uri.path().ends_with(".php") {
            return;
        }

        eprintln!("✏️ PHPMD LSP: Document changed: {}", uri);

        // Get the latest content (full sync)
        if let Some(change) = params.content_changes.first() {
            // Compress and store the updated document
            let compressed = self.compress_document(&change.text);
            let old_size = self.open_docs.read().unwrap()
                .get(&uri)
                .map(|doc| doc.compressed_data.len())
                .unwrap_or(0);
            
            self.total_memory_usage.fetch_sub(old_size, Ordering::Relaxed);
            self.open_docs.write().unwrap().insert(uri.clone(), compressed);

            // Run PHPMD and publish diagnostics
            if let Ok(diagnostics) = self.run_phpmd(&uri).await {
                eprintln!("📊 PHPMD LSP: Found {} issues after change", diagnostics.len());
                self.client
                    .publish_diagnostics(uri, diagnostics, Some(params.text_document.version))
                    .await;
            }
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri.clone();

        // Only process PHP files
        if !uri.path().ends_with(".php") {
            return;
        }

        eprintln!("💾 PHPMD LSP: Document saved: {}", uri);

        // Run PHPMD again on save to ensure we have the latest diagnostics
        if let Ok(diagnostics) = self.run_phpmd(&uri).await {
            eprintln!("📊 PHPMD LSP: Found {} issues after save", diagnostics.len());
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        
        eprintln!("📕 PHPMD LSP: Document closed: {}", uri);

        // Remove from our storage and cache
        if let Some(doc) = self.open_docs.write().unwrap().remove(&uri) {
            self.total_memory_usage.fetch_sub(doc.compressed_data.len(), Ordering::Relaxed);
        }
        self.results_cache.write().unwrap().remove(&uri);

        // Clear diagnostics for this file
        self.client.publish_diagnostics(uri, vec![], None).await;

        // Report memory usage
        let total_memory = self.total_memory_usage.load(Ordering::Relaxed);
        eprintln!("💾 PHPMD LSP: Total memory usage: {} KB", total_memory / 1024);
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        eprintln!("⚙️ PHPMD LSP: Configuration changed");

        // Try to parse the settings
        if let Ok(settings) = serde_json::from_value::<PhpmdSettings>(params.settings) {
            if let Some(ruleset) = settings.ruleset {
                eprintln!("📋 PHPMD LSP: Updated ruleset to: {}", ruleset);
                *self.ruleset.write().unwrap() = Some(ruleset);
            }
            if let Some(format) = settings.format {
                eprintln!("📋 PHPMD LSP: Updated format to: {}", format);
                *self.format.write().unwrap() = format;
            }

            // Clear the cache since settings changed
            self.results_cache.write().unwrap().clear();

            // Re-analyze all open documents with new settings
            let docs_to_reanalyze: Vec<Url> = self.open_docs.read().unwrap().keys().cloned().collect();
            for uri in docs_to_reanalyze {
                if let Ok(diagnostics) = self.run_phpmd(&uri).await {
                    self.client.publish_diagnostics(uri, diagnostics, None).await;
                }
            }
        }
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        eprintln!("📁 PHPMD LSP: Workspace folders changed");

        // Update workspace root if needed
        for added in params.event.added {
            if let Ok(path) = added.uri.to_file_path() {
                *self.workspace_root.write().unwrap() = Some(path);
                // Clear PHPMD path cache to re-discover in new workspace
                *self.phpmd_path.write().unwrap() = None;
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("🚀 PHPMD Language Server starting...");

    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::new(PhpmdLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}