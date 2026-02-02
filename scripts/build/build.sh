#!/bin/bash
# Build script wrapper for nvm-rs
# Ejecutable en Linux/macOS

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Defaults
TARGET=""
BUILD_TYPE="release"
OUTPUT_DIR="release-builds"
WITH_SELF_UPDATE=""
SKIP_CLEAN=""
VERBOSE=false

# Functions
print_section() {
    echo -e "\n${MAGENTA}$(printf '=%.0s' {1..60})${NC}"
    echo -e "${MAGENTA}$1${NC}"
    echo -e "${MAGENTA}$(printf '=%.0s' {1..60})${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ $1${NC}"
}

show_help() {
    cat << EOF
Build Script para nvm-rs (Unix/Linux/macOS)

Uso: ./scripts/build.sh [opciones]

Opciones:
  --target <target>        Target específico (windows-x64, linux-gnu-x64, macos-x64, etc.)
  --build-type <type>      Tipo de build: release (default), debug
  --output <dir>          Directorio de salida (default: release-builds)
  --with-self-update      Incluir capacidad de self-update
  --skip-clean            Saltar limpieza de build anterior
  -v, --verbose           Mostrar output detallado
  -h, --help              Mostrar esta ayuda

Ejemplos:
  ./scripts/build.sh
  ./scripts/build.sh --target linux-gnu-x64
  ./scripts/build.sh --with-self-update
  ./scripts/build.sh --target linux-gnu-x64 --with-self-update --skip-clean

Targets disponibles en este sistema:
EOF

    # Mostrar targets disponibles
    print_info "Verificando targets de Rust disponibles..."
    if command -v rustup &> /dev/null; then
        echo ""
        rustup target list --installed | sed 's/^/  • /'
    else
        echo "  (rustup no encontrado - no se pueden listar targets)"
    fi
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET="$2"
            shift 2
            ;;
        --build-type)
            BUILD_TYPE="$2"
            shift 2
            ;;
        --output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --with-self-update)
            WITH_SELF_UPDATE="-WithSelfUpdate"
            shift
            ;;
        --skip-clean)
            SKIP_CLEAN="-SkipClean"
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            print_error "Argumento desconocido: $1"
            show_help
            exit 1
            ;;
    esac
done

# Main
print_section "NVM-RS Build Script (Unix/Linux/macOS)"

print_info "Build Type: $BUILD_TYPE"
print_info "Output Directory: $OUTPUT_DIR"
if [ -n "$TARGET" ]; then
    print_info "Target: $TARGET"
fi
if [ -n "$WITH_SELF_UPDATE" ]; then
    print_info "Self-Update: Enabled"
fi
if [ "$VERBOSE" = true ]; then
    print_info "Verbose: Enabled"
fi

# Check prerequisites
print_section "Verificando Prerequisites"

# Check Rust
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo no encontrado. Instala desde https://rustup.rs"
    exit 1
fi
print_success "Cargo encontrado: $(cargo --version)"

# Check dependencies for Linux
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Check pkg-config
    if ! command -v pkg-config &> /dev/null; then
        print_warning "pkg-config no encontrado"
        print_info "Ejecuta para instalar dependencias: sudo bash ./scripts/setup-linux-build-env.sh"
        exit 1
    fi
    print_success "pkg-config encontrado: $(pkg-config --version)"

    # Check OpenSSL
    if ! pkg-config --exists openssl 2>/dev/null; then
        print_warning "OpenSSL no encontrado o no configurado"
        print_info "Ejecuta para instalar dependencias: sudo bash ./scripts/setup-linux-build-env.sh"
        exit 1
    fi
    OPENSSL_VER=$(pkg-config --modversion openssl 2>/dev/null || echo "desconocida")
    print_success "OpenSSL encontrado: $OPENSSL_VER"
fi

# Check if running in PowerShell
if command -v pwsh &> /dev/null; then
    print_info "PowerShell encontrado - usando build-releases.ps1"

    # Construir comando
    PS_CMD="./scripts/build-releases.ps1"

    if [ -n "$TARGET" ]; then
        PS_CMD="$PS_CMD -Target $TARGET"
    fi

    PS_CMD="$PS_CMD -BuildType $BUILD_TYPE"
    PS_CMD="$PS_CMD -OutputDir $OUTPUT_DIR"

    if [ -n "$WITH_SELF_UPDATE" ]; then
        PS_CMD="$PS_CMD $WITH_SELF_UPDATE"
    fi

    if [ -n "$SKIP_CLEAN" ]; then
        PS_CMD="$PS_CMD $SKIP_CLEAN"
    fi

    # Ejecutar
    pwsh -NoProfile -ExecutionPolicy Bypass -Command "& $PS_CMD"
    exit $?
fi

# Fallback: Usar cargo directamente
print_warning "PowerShell no encontrado - usando compilación básica con cargo"
print_section "Compilando"

# Convertir nombres simplificados a targets de Rust reales
if [ -n "$TARGET" ]; then
    case "$TARGET" in
        windows-x64)
            RUST_TARGET="x86_64-pc-windows-msvc"
            ;;
        windows-arm64)
            RUST_TARGET="aarch64-pc-windows-msvc"
            ;;
        linux-x64|linux-gnu-x64)
            RUST_TARGET="x86_64-unknown-linux-gnu"
            ;;
        linux-arm64|linux-gnu-arm64)
            RUST_TARGET="aarch64-unknown-linux-gnu"
            ;;
        linux-musl-x64)
            RUST_TARGET="x86_64-unknown-linux-musl"
            ;;
        linux-musl-arm64)
            RUST_TARGET="aarch64-unknown-linux-musl"
            ;;
        macos-x64)
            RUST_TARGET="x86_64-apple-darwin"
            ;;
        macos-arm64)
            RUST_TARGET="aarch64-apple-darwin"
            ;;
        *)
            # Si ya es un target de Rust válido, usarlo directamente
            RUST_TARGET="$TARGET"
            ;;
    esac
    TARGET="$RUST_TARGET"
fi

# Detectar target si no se especifica
if [ -z "$TARGET" ]; then
    # Detectar OS actual
    OS=$(uname -s)
    ARCH=$(uname -m)

    case $OS in
        Linux*)
            if [ "$ARCH" = "x86_64" ]; then
                TARGET="x86_64-unknown-linux-gnu"
            elif [ "$ARCH" = "aarch64" ]; then
                TARGET="aarch64-unknown-linux-gnu"
            else
                print_error "Arquitectura no soportada: $ARCH"
                exit 1
            fi
            ;;
        Darwin*)
            if [ "$ARCH" = "x86_64" ]; then
                TARGET="x86_64-apple-darwin"
            elif [ "$ARCH" = "arm64" ]; then
                TARGET="aarch64-apple-darwin"
            else
                print_error "Arquitectura no soportada: $ARCH"
                exit 1
            fi
            ;;
        *)
            print_error "SO no soportado: $OS"
            exit 1
            ;;
    esac

    print_info "Target detectado: $TARGET"
fi

# Build flags
CARGO_FLAGS="--target $TARGET"

if [ "$BUILD_TYPE" = "release" ]; then
    CARGO_FLAGS="$CARGO_FLAGS --release"
fi

if [ -n "$WITH_SELF_UPDATE" ]; then
    CARGO_FLAGS="$CARGO_FLAGS --features self-update"
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"
print_success "Directorio de output: $OUTPUT_DIR"

# Clean if not skipped
if [ -z "$SKIP_CLEAN" ]; then
    print_info "Limpiando builds anteriores..."
    cargo clean
fi

# Build
print_info "Compilando con: cargo build $CARGO_FLAGS"
cargo build $CARGO_FLAGS

# Find binary
if [ "$BUILD_TYPE" = "release" ]; then
    BINARY_PATH="target/$TARGET/release/nvm"
else
    BINARY_PATH="target/$TARGET/debug/nvm"
fi

if [ ! -f "$BINARY_PATH" ]; then
    print_error "Binario no encontrado: $BINARY_PATH"
    exit 1
fi

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)

# Determine OS/ARCH for output name
case $TARGET in
    x86_64-unknown-linux-gnu)
        OUT_OS="linux"
        OUT_ARCH="x64"
        ;;
    aarch64-unknown-linux-gnu)
        OUT_OS="linux"
        OUT_ARCH="arm64"
        ;;
    x86_64-unknown-linux-musl)
        OUT_OS="linux"
        OUT_ARCH="x64"
        ;;
    aarch64-unknown-linux-musl)
        OUT_OS="linux"
        OUT_ARCH="arm64"
        ;;
    x86_64-apple-darwin)
        OUT_OS="macos"
        OUT_ARCH="x64"
        ;;
    aarch64-apple-darwin)
        OUT_OS="macos"
        OUT_ARCH="arm64"
        ;;
    *)
        OUT_OS="unknown"
        OUT_ARCH="unknown"
        ;;
esac

# Output filename
OUTPUT_NAME="nvm-v${VERSION}-${OUT_OS}-${OUT_ARCH}"

# Copy to output directory
cp "$BINARY_PATH" "$OUTPUT_DIR/$OUTPUT_NAME"

# Get file size and hash
SIZE=$(stat -f%z "$OUTPUT_DIR/$OUTPUT_NAME" 2>/dev/null || stat -c%s "$OUTPUT_DIR/$OUTPUT_NAME" 2>/dev/null)
HASH=$(sha256sum "$OUTPUT_DIR/$OUTPUT_NAME" | cut -d' ' -f1)

print_success "Compilado: $OUTPUT_NAME"
print_info "Tamaño: $(numfmt --to=iec-i --suffix=B $SIZE 2>/dev/null || echo "$SIZE bytes")"
print_info "SHA256: $HASH"

# Generate checksums
print_section "Generando Checksums"

CHECKSUMS_FILE="$OUTPUT_DIR/CHECKSUMS.sha256"
echo "$HASH  $OUTPUT_NAME" > "$CHECKSUMS_FILE"
print_success "Checksums guardados: CHECKSUMS.sha256"

print_section "Build Completado"
print_success "Binario disponible en: $OUTPUT_DIR/$OUTPUT_NAME"
print_info "Hash: $HASH"
print_info "Validar con: sha256sum -c $CHECKSUMS_FILE"
