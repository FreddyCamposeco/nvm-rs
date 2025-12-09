#!/usr/bin/env bash

################################################################################
# NVM-RS Linux Build Environment Setup
#
# Este script instala todas las dependencias necesarias para compilar nvm-rs
# en sistemas Linux (Ubuntu, Debian, Fedora, etc.)
#
# Uso:
#   sudo bash ./scripts/setup-linux-build-env.sh
#   O sin sudo (usará sudo internamente):
#   bash ./scripts/setup-linux-build-env.sh
#
################################################################################

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Funciones
print_header() {
    echo -e "${CYAN}============================================================${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}============================================================${NC}"
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

# Detectar el gestor de paquetes
detect_package_manager() {
    if command -v apt-get &> /dev/null; then
        echo "apt"
    elif command -v yum &> /dev/null; then
        echo "yum"
    elif command -v dnf &> /dev/null; then
        echo "dnf"
    elif command -v pacman &> /dev/null; then
        echo "pacman"
    elif command -v apk &> /dev/null; then
        echo "apk"
    else
        echo "unknown"
    fi
}

# Función para instalar paquetes
install_packages() {
    local pm=$1
    shift
    local packages=("$@")

    case "$pm" in
        apt)
            print_info "Actualizando índice de paquetes..."
            sudo apt-get update
            print_info "Instalando paquetes: ${packages[*]}"
            sudo apt-get install -y "${packages[@]}"
            ;;
        yum)
            print_info "Instalando paquetes: ${packages[*]}"
            sudo yum install -y "${packages[@]}"
            ;;
        dnf)
            print_info "Instalando paquetes: ${packages[*]}"
            sudo dnf install -y "${packages[@]}"
            ;;
        pacman)
            print_info "Instalando paquetes: ${packages[*]}"
            sudo pacman -Sy "${packages[@]}"
            ;;
        apk)
            print_info "Instalando paquetes: ${packages[*]}"
            sudo apk add "${packages[@]}"
            ;;
        *)
            print_error "Gestor de paquetes no reconocido"
            return 1
            ;;
    esac
}

# Main
main() {
    print_header "NVM-RS Linux Build Environment Setup"

    print_info "Detectando gestor de paquetes..."
    PM=$(detect_package_manager)

    if [ "$PM" == "unknown" ]; then
        print_error "No se pudo detectar el gestor de paquetes"
        print_info "Por favor instale manualmente:"
        print_info "  - pkg-config"
        print_info "  - libssl-dev (Ubuntu/Debian) o openssl-devel (Fedora)"
        print_info "  - build-essential (Ubuntu/Debian) o build-tools (Fedora)"
        exit 1
    fi

    print_success "Gestor de paquetes detectado: $PM"
    echo ""

    # Definir paquetes según el PM
    case "$PM" in
        apt)
            print_header "Ubuntu/Debian - Instalando dependencias"
            PACKAGES=(
                "pkg-config"
                "libssl-dev"
                "build-essential"
                "curl"
            )
            print_info "Paquetes a instalar:"
            printf '  - %s\n' "${PACKAGES[@]}"
            echo ""

            read -p "¿Continuar? (s/n) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Ss]$ ]]; then
                print_warning "Instalación cancelada"
                exit 0
            fi

            install_packages "$PM" "${PACKAGES[@]}"
            ;;

        yum|dnf)
            print_header "Fedora/RHEL - Instalando dependencias"
            PACKAGES=(
                "pkgconfig"
                "openssl-devel"
                "gcc"
                "make"
                "curl"
            )
            print_info "Paquetes a instalar:"
            printf '  - %s\n' "${PACKAGES[@]}"
            echo ""

            read -p "¿Continuar? (s/n) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Ss]$ ]]; then
                print_warning "Instalación cancelada"
                exit 0
            fi

            install_packages "$PM" "${PACKAGES[@]}"
            ;;

        pacman)
            print_header "Arch Linux - Instalando dependencias"
            PACKAGES=(
                "pkg-config"
                "openssl"
                "base-devel"
                "curl"
            )
            print_info "Paquetes a instalar:"
            printf '  - %s\n' "${PACKAGES[@]}"
            echo ""

            read -p "¿Continuar? (s/n) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Ss]$ ]]; then
                print_warning "Instalación cancelada"
                exit 0
            fi

            install_packages "$PM" "${PACKAGES[@]}"
            ;;

        apk)
            print_header "Alpine Linux - Instalando dependencias"
            PACKAGES=(
                "pkgconfig"
                "openssl-dev"
                "build-base"
                "curl"
            )
            print_info "Paquetes a instalar:"
            printf '  - %s\n' "${PACKAGES[@]}"
            echo ""

            read -p "¿Continuar? (s/n) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Ss]$ ]]; then
                print_warning "Instalación cancelada"
                exit 0
            fi

            install_packages "$PM" "${PACKAGES[@]}"
            ;;
    esac

    echo ""
    print_header "Verificando instalación"

    # Verificar herramientas
    if command -v cargo &> /dev/null; then
        CARGO_VERSION=$(cargo --version)
        print_success "Cargo: $CARGO_VERSION"
    else
        print_error "Cargo no encontrado. Instale Rust desde https://rustup.rs/"
        exit 1
    fi

    if command -v pkg-config &> /dev/null; then
        PKG_CONFIG_VERSION=$(pkg-config --version)
        print_success "pkg-config: $PKG_CONFIG_VERSION"
    else
        print_error "pkg-config no se instaló correctamente"
        exit 1
    fi

    if pkg-config --exists openssl; then
        OPENSSL_VERSION=$(pkg-config --modversion openssl)
        print_success "OpenSSL: $OPENSSL_VERSION"
    else
        print_error "OpenSSL no encontrado"
        exit 1
    fi

    echo ""
    print_header "✓ Setup completado"
    echo ""
    print_success "Todas las dependencias han sido instaladas correctamente."
    echo ""
    print_info "Ya puedes compilar nvm-rs:"
    echo ""
    print_info "  bash ./scripts/build/build.sh"
    echo ""
    print_info "O para compilar un target específico:"
    echo ""
    print_info "  cargo build --target x86_64-unknown-linux-gnu --release"
    echo ""
}

# Ejecutar main
main "$@"
