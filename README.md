# PHPMD Language Server for Zed Editor

<div align="center">

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PHP](https://img.shields.io/badge/PHP-7.0%2B-777BB4?logo=php&logoColor=white)](https://php.net)
[![Zed](https://img.shields.io/badge/Zed-Editor-blue?logo=zed&logoColor=white)](https://zed.dev)
[![PHPMD](https://img.shields.io/badge/PHPMD-2.15%2B-green)](https://phpmd.org)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://rust-lang.org)

**Real-time PHP code quality analysis powered by PHP Mess Detector**

[Features](#-features) • [Installation](#-installation) • [Configuration](#-configuration) • [Architecture](#-architecture) • [Contributing](#-contributing)

</div>

---

## 📋 Overview

The PHPMD Language Server brings comprehensive code quality analysis to Zed Editor through the Language Server Protocol (LSP). It provides real-time feedback on potential bugs, suboptimal code, overcomplicated expressions, and unused code elements as you write PHP.

### Why PHPMD LSP?

- **🚀 Instant Feedback** - See code quality issues as you type, not after you commit
- **⚡ High Performance** - Optimized with async processing, LZ4 compression, and smart caching
- **🎯 Zero Configuration** - Works out of the box with sensible defaults
- **🔧 Highly Configurable** - Customize rules to match your team's standards
- **📦 Self-Contained** - Includes bundled PHPMD binary, no external dependencies required

## ✨ Features

### Core Capabilities

#### 🔍 **Real-Time Code Analysis**
- Detects code quality issues instantly as you type
- Smart diagnostic ranges that skip leading whitespace for cleaner underlines
- Severity-based highlighting (errors, warnings, info)
- Rule-specific documentation links

#### ⚡ **Performance Optimizations**
- **LZ4 Compression** - Reduces memory usage by ~85% for large files
- **Smart Caching** - Avoids re-analyzing unchanged files
- **Concurrent Processing** - Analyzes up to 4 files simultaneously
- **Async Architecture** - Non-blocking execution keeps editor responsive
- **Timeout Protection** - 10-second safeguard prevents hanging on complex files

#### 🎨 **Intelligent Diagnostics**
- **Context-Aware Ranges** - Different highlighting for class, method, and property issues
- **Whitespace Trimming** - Underlines start at first code character, not indentation
- **Property Detection** - Correctly highlights property violations even when PHPMD reports class-level
- **Rule Categorization** - Automatic severity mapping based on rule priority

### Configuration Features

#### 🔧 **Flexible Configuration**
- **Auto-Discovery** - Finds `phpmd.xml`, `phpmd.xml.dist`, `.phpmd.xml`, or `.phpmd.xml.dist` automatically
- **XML Validation** - Validates config files before use, skipping invalid XML files
- **Smart Fallback** - Uses all PHPMD rulesets when no valid config file is found
- **Live Reloading** - Configuration changes apply instantly without restart
- **Multi-Source** - Configure via project files, Zed settings, or environment variables
- **Auto-Recovery** - Gracefully handles deleted or invalid config files

#### 📦 **Smart PHPMD Detection**
Priority order for finding PHPMD executable:
1. Project's `vendor/bin/phpmd` (respects composer dependencies)
2. Custom path from settings
3. Bundled PHPMD 2.15+ PHAR (always available fallback)
4. System PATH installation

### Reliability Features

#### 🛡️ **Robust Error Handling**
- **Process Management** - Automatic cleanup of zombie processes
- **Memory Monitoring** - Tracks and logs memory usage for optimization
- **Graceful Degradation** - Falls back to defaults when configs fail
- **Comprehensive Logging** - Detailed debug output for troubleshooting

## 📦 Installation

### Via Zed Extensions Panel (Recommended)

1. Open Zed Editor
2. Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows)
3. Search for "PHPMD"
4. Click Install

### Manual Installation (Development)

```bash
# Clone the repository
git clone https://github.com/GeneaLabs/zed-phpmd-lsp.git
cd zed-phpmd-lsp

# Build the extension
cargo build --release

# Build the LSP server
cd lsp-server
cargo build --release
```

## 🚀 Quick Start

### 1. Enable the Language Server

Add to your Zed `settings.json` (open with `Cmd+,` or `Ctrl+,`):

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["phpmd", "intelephense"]
    }
  }
}
```

### 2. Start Coding

The extension automatically analyzes PHP files as you open and edit them:

```php
<?php
namespace App\Services;

class UserService
{
    private $unused; // 🔍 Unused private field
    
    public function complexMethod($a, $b, $c, $d, $e, $f) // 🔍 Too many parameters
    {
        if ($a == true) { // 🔍 Use strict comparison
            goto end; // 🔍 Avoid goto statements
        }
        
        eval($userInput); // 🔍 Security risk: eval usage
        
        end:
        return $result;
    }
    
    public function m() { // 🔍 Method name too short
        // Complex logic here...
    }
}
```

## ⚙️ Configuration

### Configuration Priority

The extension uses this priority order (highest to lowest):

1. **Project Config Files** - `phpmd.xml`, `phpmd.xml.dist`, `.phpmd.xml`
2. **Zed Settings** - User or project-specific settings.json
3. **Environment Variables** - `PHPMD_RULESETS`, `PHPMD_PATH`
4. **Defaults** - Built-in rulesets: cleancode, codesize, controversial, design, naming, unusedcode

### Available Rulesets

| Ruleset | Description | Key Rules |
|---------|-------------|-----------|
| **cleancode** | Clean code practices | BooleanArgumentFlag, ElseExpression, StaticAccess |
| **codesize** | Complexity and size limits | CyclomaticComplexity, NPathComplexity, ExcessiveMethodLength |
| **controversial** | Debated best practices | Superglobals, CamelCaseParameterName |
| **design** | Software design principles | ExitExpression, EvalExpression, GotoStatement |
| **naming** | Naming conventions | ShortVariable, LongVariable, ShortMethodName |
| **unusedcode** | Dead code detection | UnusedPrivateField, UnusedLocalVariable, UnusedPrivateMethod |

### Configuration Examples

#### Basic Zed Settings

```json
{
  "lsp": {
    "phpmd": {
      "settings": {
        "rulesets": "cleancode,codesize,design"
      }
    }
  }
}
```

#### Advanced Project Configuration

Create `phpmd.xml` in your project root:

```xml
<?xml version="1.0"?>
<ruleset name="MyProject"
         xmlns="http://pmd.sf.net/ruleset/1.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://pmd.sf.net/ruleset/1.0.0
                             http://pmd.sf.net/ruleset_xml_schema.xsd">
    
    <description>Custom rules for MyProject</description>
    
    <!-- Import specific rules -->
    <rule ref="rulesets/cleancode.xml">
        <exclude name="ElseExpression"/>
    </rule>
    
    <rule ref="rulesets/codesize.xml/CyclomaticComplexity">
        <properties>
            <property name="reportLevel" value="15"/>
        </properties>
    </rule>
    
    <rule ref="rulesets/naming.xml/ShortVariable">
        <properties>
            <property name="minimum" value="2"/>
            <property name="exceptions" value="id,db,em,i,j,k"/>
        </properties>
    </rule>
    
    <!-- Exclude patterns -->
    <exclude-pattern>*/vendor/*</exclude-pattern>
    <exclude-pattern>*/tests/*</exclude-pattern>
    <exclude-pattern>*/database/migrations/*</exclude-pattern>
</ruleset>
```

#### Per-Project Settings

Create `.zed/settings.json` in your project:

```json
{
  "lsp": {
    "phpmd": {
      "settings": {
        "rulesets": "./config/phpmd-strict.xml",
        "phpmdPath": "./vendor/bin/phpmd"
      }
    }
  }
}
```

## 🏗️ Architecture

### System Overview

```
┌─────────────────┐     LSP Protocol      ┌──────────────────┐
│                 │ ◄──────────────────► │                  │
│   Zed Editor    │                      │   PHPMD LSP      │
│                 │                      │    Server        │
└─────────────────┘                      └──────────────────┘
        │                                         │
        │                                         │
        ▼                                         ▼
┌─────────────────┐                      ┌──────────────────┐
│  PHP Files      │                      │   PHPMD Binary   │
│  in Workspace   │                      │   (Analysis)     │
└─────────────────┘                      └──────────────────┘
```

### Key Components

#### **LSP Server** (`lsp-server/src/main.rs`)
- **Tower-LSP Framework** - Async LSP implementation in Rust
- **Tokio Runtime** - High-performance async execution
- **LZ4 Compression** - Memory-efficient document storage
- **Semaphore Control** - Limits concurrent PHPMD processes

#### **Memory Management**
```rust
// Document compression reduces memory by ~85%
CompressedDocument {
    compressed_data: Vec<u8>,      // LZ4 compressed content
    original_size: usize,           // Original file size
    checksum: u64,                  // Content hash for cache validation
    compression_ratio: f32,         // Monitoring compression efficiency
}
```

#### **Diagnostic Processing Pipeline**
1. **File Change Detection** - Debounced to avoid excessive processing
2. **Content Compression** - LZ4 compression for memory efficiency
3. **PHPMD Execution** - Isolated process with timeout protection
4. **JSON Parsing** - Robust extraction from mixed PHPMD output
5. **Range Calculation** - Smart positioning with whitespace trimming
6. **Cache Storage** - Results cached with content checksums

### Performance Characteristics

- **Memory Usage**: ~15-20MB base + ~0.2MB per open file (compressed)
- **Analysis Speed**: 50-200ms for typical files (1000 lines)
- **Concurrent Files**: Up to 4 simultaneous analyses
- **Cache Hit Rate**: ~70-80% in typical editing sessions
- **Compression Ratio**: 85-90% size reduction for source files

## 🐛 Troubleshooting

### Common Issues

#### Diagnostics Not Appearing

1. **Verify language server is enabled:**
   ```json
   {
     "languages": {
       "PHP": {
         "language_servers": ["phpmd"]
       }
     }
   }
   ```

2. **Check debug output:**
   - View → Toggle Log Panel
   - Look for `PHPMD LSP:` prefixed messages

3. **Test PHPMD manually:**
   ```bash
   phpmd path/to/file.php text cleancode
   ```

#### Performance Issues

1. **Reduce concurrent analyses:**
   ```json
   {
     "lsp": {
       "phpmd": {
         "settings": {
           "maxConcurrent": 2
         }
       }
     }
   }
   ```

2. **Simplify rulesets:**
   - Start with single ruleset: `"rulesets": "cleancode"`
   - Add more gradually to identify performance impact

#### Custom Rules Not Working

1. **Validate XML syntax:**
   ```bash
   xmllint --noout phpmd.xml
   ```

2. **Check file paths:**
   - Use absolute paths for testing
   - Ensure relative paths are from project root

3. **Test configuration:**
   ```bash
   phpmd . text phpmd.xml --verbose
   ```

### Debug Mode

Enable verbose logging in Zed:

```json
{
  "lsp": {
    "phpmd": {
      "settings": {
        "logLevel": "debug"
      }
    }
  }
}
```

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone and setup
git clone https://github.com/GeneaLabs/zed-phpmd-lsp.git
cd zed-phpmd-lsp

# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build everything
cargo build --release
cd lsp-server && cargo build --release

# Run tests
cargo test
```

### Areas for Contribution

- 🪟 Windows testing and compatibility
- 🐧 Linux distribution testing
- 📝 Additional ruleset templates
- 🌍 Internationalization support
- 🧪 Test coverage improvements
- 📚 Documentation translations

## 📚 Resources

- [PHPMD Documentation](https://phpmd.org/documentation/index.html)
- [PHPMD Rules Reference](https://phpmd.org/rules/index.html)
- [Zed Extensions Guide](https://zed.dev/docs/extensions)
- [Language Server Protocol Spec](https://microsoft.github.io/language-server-protocol/)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Third-Party Licenses

- **PHPMD** - BSD-3-Clause License (bundled as PHAR)
- **Rust Dependencies** - Various permissive licenses

See [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md) for complete attribution.

## 🙏 Acknowledgments

- **[Manuel Pichler](https://github.com/manuelpichler)** - Creator of PHPMD
- **[PHPMD Contributors](https://github.com/phpmd/phpmd/graphs/contributors)** - For the excellent code quality tool
- **[Zed Industries](https://zed.dev)** - For the blazing-fast editor and extension API
- **[Tower-LSP Contributors](https://github.com/ebkalderon/tower-lsp)** - For the robust LSP framework
- **PHP Community** - For continuous feedback and support

---

<div align="center">

**Made with ❤️ for the PHP community**

[Report Bug](https://github.com/GeneaLabs/zed-phpmd-lsp/issues) • [Request Feature](https://github.com/GeneaLabs/zed-phpmd-lsp/issues) • [Discussions](https://github.com/GeneaLabs/zed-phpmd-lsp/discussions)

</div>