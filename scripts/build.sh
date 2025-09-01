#!/usr/bin/env bash

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the directory of this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo -e "${GREEN}🔨 Building PHPMD LSP Extension${NC}"
echo "Project root: $PROJECT_ROOT"

# Change to project root
cd "$PROJECT_ROOT"

# Function to build LSP server for a specific target
build_lsp_server() {
    local target=$1
    local output_name=$2
    
    echo -e "${YELLOW}Building LSP server for $target...${NC}"
    
    cd "$PROJECT_ROOT/lsp-server"
    
    if [[ "$target" == "local" ]]; then
        cargo build --release
        cp "target/release/phpmd-lsp-server" "../$output_name"
    else
        cargo build --release --target "$target"
        if [[ "$target" == *"windows"* ]]; then
            cp "target/$target/release/phpmd-lsp-server.exe" "../$output_name"
        else
            cp "target/$target/release/phpmd-lsp-server" "../$output_name"
        fi
    fi
    
    echo -e "${GREEN}✓ Built $output_name${NC}"
}

# Function to download PHPMD PHAR
download_phpmd_phar() {
    echo -e "${YELLOW}Downloading PHPMD PHAR...${NC}"
    
    # PHPMD latest release URL
    PHPMD_URL="https://github.com/phpmd/phpmd/releases/latest/download/phpmd.phar"
    
    cd "$PROJECT_ROOT"
    
    if [[ ! -f "phpmd.phar" ]]; then
        curl -L -o phpmd.phar "$PHPMD_URL"
        chmod +x phpmd.phar
        echo -e "${GREEN}✓ Downloaded phpmd.phar${NC}"
    else
        echo -e "${GREEN}✓ phpmd.phar already exists${NC}"
    fi
}

# Build based on arguments
case "${1:-all}" in
    "local")
        echo -e "${YELLOW}Building for local development...${NC}"
        
        # Build the LSP server
        build_lsp_server "local" "phpmd-lsp-server"
        
        # Download PHPMD PHAR
        download_phpmd_phar
        
        # Build the extension WASM
        echo -e "${YELLOW}Building extension WASM...${NC}"
        cd "$PROJECT_ROOT"
        cargo build --release --target wasm32-wasi
        cp target/wasm32-wasi/release/zed_phpmd_lsp.wasm extension.wasm
        
        echo -e "${GREEN}✓ Local build complete!${NC}"
        ;;
        
    "macos-arm64")
        build_lsp_server "aarch64-apple-darwin" "phpmd-lsp-server-macos-arm64"
        ;;
        
    "macos-x64")
        build_lsp_server "x86_64-apple-darwin" "phpmd-lsp-server-macos-x64"
        ;;
        
    "linux-x64")
        build_lsp_server "x86_64-unknown-linux-gnu" "phpmd-lsp-server-linux-x64"
        ;;
        
    "linux-arm64")
        build_lsp_server "aarch64-unknown-linux-gnu" "phpmd-lsp-server-linux-arm64"
        ;;
        
    "windows-x64")
        build_lsp_server "x86_64-pc-windows-gnu" "phpmd-lsp-server-windows-x64.exe"
        ;;
        
    "windows-arm64")
        build_lsp_server "aarch64-pc-windows-msvc" "phpmd-lsp-server-windows-arm64.exe"
        ;;
        
    "release")
        echo -e "${YELLOW}Building all release targets...${NC}"
        
        # Download PHPMD PHAR first
        download_phpmd_phar
        
        # Build all targets
        "$0" macos-arm64
        "$0" macos-x64
        "$0" linux-x64
        "$0" linux-arm64
        "$0" windows-x64
        
        # Build extension WASM
        echo -e "${YELLOW}Building extension WASM...${NC}"
        cd "$PROJECT_ROOT"
        cargo build --release --target wasm32-wasi
        cp target/wasm32-wasi/release/zed_phpmd_lsp.wasm extension.wasm
        
        # Create release archives
        echo -e "${YELLOW}Creating release archives...${NC}"
        cd "$PROJECT_ROOT"
        
        # Create archives for each platform binary
        for binary in phpmd-lsp-server-*; do
            if [[ -f "$binary" ]]; then
                tar -czf "$binary.tar.gz" "$binary"
                echo -e "${GREEN}✓ Created $binary.tar.gz${NC}"
            fi
        done
        
        # Create PHPMD PHAR archive
        tar -czf phpmd.phar.tar.gz phpmd.phar
        echo -e "${GREEN}✓ Created phpmd.phar.tar.gz${NC}"
        
        echo -e "${GREEN}🎉 Release build complete!${NC}"
        echo -e "${GREEN}Release assets created:${NC}"
        ls -lh *.tar.gz extension.wasm
        ;;
        
    "clean")
        echo -e "${YELLOW}Cleaning build artifacts...${NC}"
        
        cd "$PROJECT_ROOT"
        rm -rf target/
        rm -rf lsp-server/target/
        rm -f phpmd-lsp-server*
        rm -f *.tar.gz
        rm -f extension.wasm
        rm -f phpmd.phar
        
        echo -e "${GREEN}✓ Clean complete${NC}"
        ;;
        
    "test")
        echo -e "${YELLOW}Running tests...${NC}"
        
        # Test the extension
        cd "$PROJECT_ROOT"
        cargo test
        
        # Test the LSP server
        cd "$PROJECT_ROOT/lsp-server"
        cargo test
        
        echo -e "${GREEN}✓ Tests complete${NC}"
        ;;
        
    *)
        echo "Usage: $0 [local|macos-arm64|macos-x64|linux-x64|linux-arm64|windows-x64|release|clean|test]"
        echo ""
        echo "  local         - Build for local development"
        echo "  macos-arm64   - Build for macOS ARM64"
        echo "  macos-x64     - Build for macOS x64"
        echo "  linux-x64     - Build for Linux x64"
        echo "  linux-arm64   - Build for Linux ARM64"
        echo "  windows-x64   - Build for Windows x64"
        echo "  release       - Build all release targets and create archives"
        echo "  clean         - Remove all build artifacts"
        echo "  test          - Run tests"
        echo ""
        echo "Default: all (builds everything)"
        exit 1
        ;;
esac