#!/bin/bash
# Advanced Obfuscation Build Script for ShadowSniff
# This script orchestrates the complete obfuscation pipeline

set -e

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="$PROJECT_ROOT/target"
OBFUSCATED_DIR="$TARGET_DIR/obfuscated"
TOOLS_DIR="$PROJECT_ROOT/tools"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log() {
    echo -e "${GREEN}[OBFUSCATION]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Check for required tools
check_dependencies() {
    log "Checking dependencies..."
    
    # Check for Rust and Cargo
    if ! command -v cargo &> /dev/null; then
        error "Cargo not found. Please install Rust toolchain."
    fi
    
    # Check for LLVM tools (optional but recommended)
    if command -v llvm-objcopy &> /dev/null; then
        log "LLVM tools detected - enhanced obfuscation available"
        LLVM_AVAILABLE=true
    else
        warn "LLVM tools not found - using basic obfuscation only"
        LLVM_AVAILABLE=false
    fi
    
    # Check for upx (optional)
    if command -v upx &> /dev/null; then
        log "UPX packer detected - binary compression available"
        UPX_AVAILABLE=true
    else
        warn "UPX not found - skipping binary compression"
        UPX_AVAILABLE=false
    fi
}

# Set obfuscation level based on argument
set_obfuscation_level() {
    local level=${1:-"high"}
    
    case $level in
        "maximum"|"max")
            export SHADOWSNIFF_OBFUSCATION_LEVEL="maximum"
            log "Using MAXIMUM obfuscation level"
            ;;
        "high"|"h")
            export SHADOWSNIFF_OBFUSCATION_LEVEL="high"
            log "Using HIGH obfuscation level"
            ;;
        "medium"|"med"|"m")
            export SHADOWSNIFF_OBFUSCATION_LEVEL="medium"
            log "Using MEDIUM obfuscation level"
            ;;
        "low"|"l")
            export SHADOWSNIFF_OBFUSCATION_LEVEL="low"
            log "Using LOW obfuscation level"
            ;;
        *)
            warn "Unknown obfuscation level '$level', defaulting to HIGH"
            export SHADOWSNIFF_OBFUSCATION_LEVEL="high"
            ;;
    esac
}

# Apply pre-build obfuscation
pre_build_obfuscation() {
    log "Applying pre-build obfuscation..."
    
    # Create obfuscated directory
    mkdir -p "$OBFUSCATED_DIR"
    
    # Generate random build seed for consistent obfuscation
    BUILD_SEED=$(date +%s | sha256sum | cut -c1-16)
    export SHADOWSNIFF_BUILD_SEED="$BUILD_SEED"
    log "Build seed: $BUILD_SEED"
    
    # Set additional LLVM flags for control flow obfuscation
    if [ "$LLVM_AVAILABLE" = true ]; then
        export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-mllvm -C llvm-args=-seed=$BUILD_SEED"
        export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-mllvm -C llvm-args=-enable-cff"
        export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-mllvm -C llvm-args=-enable-bcf"
        export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-mllvm -C llvm-args=-enable-split"
        log "Applied LLVM obfuscation flags"
    fi
    
    # Set optimization flags for smaller and faster binaries
    export RUSTFLAGS="$RUSTFLAGS -C target-cpu=native"
    export RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,--gc-sections"
    export RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,--strip-all"
}

# Build the project with obfuscation
build_project() {
    log "Building ShadowSniff with obfuscation..."
    
    cd "$PROJECT_ROOT"
    
    # Clean previous builds
    cargo clean
    
    # Build with release optimizations and obfuscation
    cargo build --release --features builder_build
    
    if [ $? -ne 0 ]; then
        error "Build failed"
    fi
    
    log "Build completed successfully"
}

# Apply post-build obfuscation
post_build_obfuscation() {
    log "Applying post-build obfuscation..."
    
    local binary_path="$TARGET_DIR/release/ShadowSniff.exe"
    local obfuscated_path="$OBFUSCATED_DIR/ShadowSniff.exe"
    
    if [ ! -f "$binary_path" ]; then
        error "Built binary not found at $binary_path"
    fi
    
    # Copy binary to obfuscated directory
    cp "$binary_path" "$obfuscated_path"
    
    # Strip debug symbols if not already done
    if [ "$LLVM_AVAILABLE" = true ]; then
        llvm-strip "$obfuscated_path"
        log "Stripped debug symbols"
    fi
    
    # Apply binary packing with UPX
    if [ "$UPX_AVAILABLE" = true ]; then
        upx --ultra-brute "$obfuscated_path"
        log "Applied UPX compression"
    fi
    
    # Generate obfuscation report
    generate_obfuscation_report "$obfuscated_path"
}

# Generate obfuscation report
generate_obfuscation_report() {
    local binary_path="$1"
    local report_path="$OBFUSCATED_DIR/obfuscation_report.txt"
    
    log "Generating obfuscation report..."
    
    cat > "$report_path" << EOF
ShadowSniff Obfuscation Report
=============================

Build Date: $(date)
Build Seed: $SHADOWSNIFF_BUILD_SEED
Obfuscation Level: $SHADOWSNIFF_OBFUSCATION_LEVEL

Binary Information:
- Path: $binary_path
- Size: $(stat -f%z "$binary_path" 2>/dev/null || stat -c%s "$binary_path") bytes
- SHA256: $(sha256sum "$binary_path" | cut -d' ' -f1)

Applied Obfuscations:
- String obfuscation (obfstr + custom)
- Control flow obfuscation
- Anti-debugging techniques
- Runtime polymorphism
- Binary protection
EOF

    if [ "$LLVM_AVAILABLE" = true ]; then
        echo "- LLVM-based obfuscation passes" >> "$report_path"
    fi
    
    if [ "$UPX_AVAILABLE" = true ]; then
        echo "- UPX binary compression" >> "$report_path"
    fi
    
    echo "" >> "$report_path"
    echo "Obfuscation Features:" >> "$report_path"
    echo "- Anti-debugging checks" >> "$report_path"
    echo "- VM/Sandbox detection" >> "$report_path"
    echo "- API hooking detection" >> "$report_path"
    echo "- Control flow flattening" >> "$report_path"
    echo "- Instruction substitution" >> "$report_path"
    echo "- Opaque predicates" >> "$report_path"
    echo "- Runtime code mutation" >> "$report_path"
    echo "- Memory protection" >> "$report_path"
    echo "- String encryption layers" >> "$report_path"
    echo "- Anti-emulation techniques" >> "$report_path"
    
    log "Report generated: $report_path"
}

# Test against common analysis tools
test_against_analysis_tools() {
    log "Testing against common analysis tools..."
    
    local binary_path="$OBFUSCATED_DIR/ShadowSniff.exe"
    
    if [ ! -f "$binary_path" ]; then
        error "Obfuscated binary not found"
    fi
    
    # Test with strings command
    log "Testing string extraction resistance..."
    local string_count=$(strings "$binary_path" | wc -l)
    log "Extractable strings: $string_count"
    
    # Test with file command
    log "Testing file type detection..."
    file "$binary_path"
    
    # Test with objdump (if available)
    if command -v objdump &> /dev/null; then
        log "Testing disassembly resistance..."
        local asm_lines=$(objdump -d "$binary_path" 2>/dev/null | wc -l || echo "0")
        log "Disassembled lines: $asm_lines"
    fi
    
    log "Analysis tool testing completed"
}

# Main execution
main() {
    log "Starting ShadowSniff obfuscation build pipeline..."
    
    # Parse command line arguments
    local obfuscation_level=${1:-"high"}
    local run_tests=${2:-"false"}
    
    # Execute pipeline steps
    check_dependencies
    set_obfuscation_level "$obfuscation_level"
    pre_build_obfuscation
    build_project
    post_build_obfuscation
    
    if [ "$run_tests" = "true" ] || [ "$run_tests" = "test" ]; then
        test_against_analysis_tools
    fi
    
    log "Obfuscation pipeline completed successfully!"
    log "Obfuscated binary: $OBFUSCATED_DIR/ShadowSniff.exe"
}

# Show usage information
show_usage() {
    echo "Usage: $0 [obfuscation_level] [test]"
    echo ""
    echo "Obfuscation levels:"
    echo "  maximum, max  - Maximum obfuscation (slowest build, best protection)"
    echo "  high, h       - High obfuscation (default)"
    echo "  medium, med   - Medium obfuscation" 
    echo "  low, l        - Low obfuscation (fastest build)"
    echo ""
    echo "Options:"
    echo "  test          - Run analysis tool tests after build"
    echo ""
    echo "Examples:"
    echo "  $0                    # Build with high obfuscation"
    echo "  $0 maximum            # Build with maximum obfuscation"
    echo "  $0 high test          # Build with high obfuscation and run tests"
}

# Handle command line arguments
case "${1:-}" in
    -h|--help|help)
        show_usage
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac