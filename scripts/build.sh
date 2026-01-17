#!/usr/bin/env bash
#
# Optimized build script for rust-calc (final size ~555KB)
# Usage: ./build.sh [OPTIONS]
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"
BINARY_NAME="rust-calc"

cd "$PROJECT_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m'

# Detect platform and target
detect_target() {
    case "$(uname -s)-$(uname -m)" in
        Darwin-arm64)  echo "aarch64-apple-darwin" ;;
        Darwin-x86_64) echo "x86_64-apple-darwin" ;;
        Linux-x86_64)  echo "x86_64-unknown-linux-gnu" ;;
        Linux-aarch64) echo "aarch64-unknown-linux-gnu" ;;
        MINGW*|MSYS*)  echo "x86_64-pc-windows-msvc" ;;
        *)             echo "" ;;
    esac
}

detect_platform() {
    case "$(uname -s)" in
        Darwin*)  echo "macos" ;;
        Linux*)   echo "linux" ;;
        MINGW*|MSYS*) echo "windows" ;;
        *)        echo "unix" ;;
    esac
}

TARGET=$(detect_target)
PLATFORM=$(detect_platform)

# Parse arguments
PROFILE="release"
CLEAN=false
NO_COMPRESS=false
NO_OPTIMIZE=false
INSTALL=false

show_help() {
    echo "Optimized build script for rust-calc"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --release       Build optimized for size (default)"
    echo "  --fast          Build optimized for speed"
    echo "  --debug         Build with debug info"
    echo "  --clean         Clean before building"
    echo "  --no-compress   Skip UPX compression"
    echo "  --no-optimize   Skip nightly + build-std (use stable)"
    echo "  --install       Install to /usr/local/bin"
    echo "  -h, --help      Show this help"
    echo ""
    echo "Output: ~555KB with full optimization"
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)     PROFILE="release"; shift ;;
        --fast)        PROFILE="release-fast"; shift ;;
        --debug)       PROFILE="dev"; shift ;;
        --clean)       CLEAN=true; shift ;;
        --no-compress) NO_COMPRESS=true; shift ;;
        --no-optimize) NO_OPTIMIZE=true; shift ;;
        --install)     INSTALL=true; shift ;;
        -h|--help)     show_help; exit 0 ;;
        *)             echo -e "${RED}Unknown option: $1${NC}"; exit 1 ;;
    esac
done

# Check nightly
check_nightly() {
    rustup run nightly rustc --version &>/dev/null
}

install_nightly() {
    echo -e "${YELLOW}Installing nightly toolchain...${NC}"
    rustup install nightly
    rustup component add rust-src --toolchain nightly
}

# Clean
if [ "$CLEAN" = true ]; then
    echo -e "${YELLOW}Cleaning...${NC}"
    cargo clean
    rm -rf "$DIST_DIR"
fi

# Determine build mode
USE_NIGHTLY=false
if [ "$NO_OPTIMIZE" = false ] && [ "$PROFILE" != "dev" ] && [ -n "$TARGET" ]; then
    USE_NIGHTLY=true
    if ! check_nightly; then
        install_nightly
    fi
fi

# Build
if [ "$PROFILE" = "dev" ]; then
    echo -e "${CYAN}Building debug...${NC}"
    cargo build
    SOURCE_DIR="$PROJECT_ROOT/target/debug"
elif [ "$USE_NIGHTLY" = true ]; then
    echo -e "${CYAN}Building with nightly + build-std (optimized)...${NC}"
    cargo +nightly build --release \
        -Z build-std=std,panic_abort \
        --target "$TARGET"
    SOURCE_DIR="$PROJECT_ROOT/target/$TARGET/release"
else
    echo -e "${CYAN}Building with stable...${NC}"
    cargo build --profile "$PROFILE"
    SOURCE_DIR="$PROJECT_ROOT/target/$PROFILE"
fi

# Create dist
mkdir -p "$DIST_DIR"

# Paths
if [ "$PLATFORM" = "windows" ]; then
    SOURCE_PATH="$SOURCE_DIR/${BINARY_NAME}.exe"
    DEST_PATH="$DIST_DIR/${BINARY_NAME}.exe"
else
    SOURCE_PATH="$SOURCE_DIR/$BINARY_NAME"
    DEST_PATH="$DIST_DIR/${BINARY_NAME}-${PLATFORM}"
fi

if [ ! -f "$SOURCE_PATH" ]; then
    echo -e "${RED}Binary not found: $SOURCE_PATH${NC}"
    exit 1
fi

# Get size (cross-platform)
get_size() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        stat -f%z "$1"
    else
        stat -c%s "$1"
    fi
}

ORIGINAL_SIZE=$(get_size "$SOURCE_PATH")

# UPX compression
if [ "$NO_COMPRESS" = false ] && [ "$PROFILE" != "dev" ] && command -v upx &>/dev/null; then
    echo -e "${CYAN}Compressing with UPX...${NC}"
    rm -f "$DEST_PATH"
    if upx --best --lzma "$SOURCE_PATH" -o "$DEST_PATH" 2>/dev/null; then
        COMPRESSED_SIZE=$(get_size "$DEST_PATH")
        RATIO=$(echo "scale=1; $COMPRESSED_SIZE * 100 / $ORIGINAL_SIZE" | bc)
        echo -e "${GREEN}Compressed: ${RATIO}% of original${NC}"
    else
        echo -e "${YELLOW}UPX failed, copying uncompressed${NC}"
        cp "$SOURCE_PATH" "$DEST_PATH"
    fi
else
    if [ "$NO_COMPRESS" = false ] && [ "$PROFILE" != "dev" ]; then
        echo -e "${GRAY}UPX not found. Install: brew install upx (macOS) or apt install upx (Linux)${NC}"
    fi
    cp "$SOURCE_PATH" "$DEST_PATH"
fi

# Strip (non-debug, non-Windows)
if [ "$PROFILE" != "dev" ] && [ "$PLATFORM" != "windows" ]; then
    strip "$DEST_PATH" 2>/dev/null || true
fi

chmod +x "$DEST_PATH"

# Install
if [ "$INSTALL" = true ]; then
    INSTALL_DIR="/usr/local/bin"
    echo ""
    if [ -w "$INSTALL_DIR" ]; then
        cp "$DEST_PATH" "$INSTALL_DIR/$BINARY_NAME"
    else
        echo -e "${YELLOW}Installing requires sudo...${NC}"
        sudo cp "$DEST_PATH" "$INSTALL_DIR/$BINARY_NAME"
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi
    echo -e "${CYAN}Installed to: $INSTALL_DIR/$BINARY_NAME${NC}"
fi

# Output
FINAL_SIZE=$(get_size "$DEST_PATH")
SIZE_KB=$((FINAL_SIZE / 1024))

echo ""
echo -e "${GRAY}========================================${NC}"
echo -e "  Output: ${DEST_PATH}"
echo -e "  Size:   ${GREEN}${SIZE_KB} KB${NC}"
echo -e "${GRAY}========================================${NC}"
echo ""
