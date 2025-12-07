#!/usr/bin/env bash
# Script de instalación de nvm-rs para Linux/macOS
# Uso: curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/install.sh | bash

set -e

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Funciones de output
info() { echo -e "${CYAN}$1${NC}"; }
success() { echo -e "${GREEN}$1${NC}"; }
warning() { echo -e "${YELLOW}$1${NC}"; }
error() { echo -e "${RED}$1${NC}"; exit 1; }

# Parámetros
VERSION="${NVM_VERSION:-latest}"
INSTALL_DIR="${NVM_INSTALL_DIR:-$HOME/.local/bin}"
WITH_SELF_UPDATE="${NVM_WITH_SELF_UPDATE:-false}"

info "=== Instalador de nvm-rs para Linux/macOS ==="
echo ""

# Detectar sistema operativo
OS="$(uname -s)"
case "$OS" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    *)          error "Sistema operativo no soportado: $OS";;
esac

# Detectar arquitectura
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64)     ARCH="x64";;
    aarch64|arm64) ARCH="arm64";;
    *)          error "Arquitectura no soportada: $ARCH";;
esac

info "Sistema detectado: $OS $ARCH"
echo ""

# Obtener información de la release
info "Obteniendo información de la versión..."
if [ "$VERSION" = "latest" ]; then
    API_URL="https://api.github.com/repos/FreddyCamposeco/nvm-rs/releases/latest"
else
    API_URL="https://api.github.com/repos/FreddyCamposeco/nvm-rs/releases/tags/$VERSION"
fi

RELEASE_DATA=$(curl -fsSL "$API_URL" -H "User-Agent: nvm-rs-installer") || error "Error al obtener información de la release"
RELEASE_VERSION=$(echo "$RELEASE_DATA" | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4)

if [ -z "$RELEASE_VERSION" ]; then
    error "No se pudo obtener la versión de la release"
fi

info "Versión a instalar: $RELEASE_VERSION"
echo ""

# Determinar nombre del asset
SUFFIX=""
if [ "$WITH_SELF_UPDATE" = "true" ]; then
    SUFFIX="-self-update"
fi

ASSET_NAME="nvm-${RELEASE_VERSION}-${OS}-${ARCH}${SUFFIX}"
info "Asset a descargar: $ASSET_NAME"

# Buscar URL de descarga
DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep -o "\"browser_download_url\": \"[^\"]*$ASSET_NAME\"" | cut -d'"' -f4)

if [ -z "$DOWNLOAD_URL" ]; then
    error "Asset $ASSET_NAME no encontrado en la release"
fi

# Crear directorio de instalación
info "Directorio de instalación: $INSTALL_DIR"
if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
    success "✓ Directorio de instalación creado"
fi

# Descargar binario
TEMP_FILE=$(mktemp)
info ""
info "Descargando $ASSET_NAME..."
info "URL: $DOWNLOAD_URL"

if command -v wget &> /dev/null; then
    wget -q --show-progress -O "$TEMP_FILE" "$DOWNLOAD_URL" || error "Error al descargar el binario"
elif command -v curl &> /dev/null; then
    curl -fL --progress-bar -o "$TEMP_FILE" "$DOWNLOAD_URL" || error "Error al descargar el binario"
else
    error "Se requiere 'wget' o 'curl' para descargar el binario"
fi

success "✓ Descarga completada"

# Verificar checksum
info ""
info "Verificando integridad del archivo..."
if command -v sha256sum &> /dev/null; then
    CHECKSUM=$(sha256sum "$TEMP_FILE" | cut -d' ' -f1)
elif command -v shasum &> /dev/null; then
    CHECKSUM=$(shasum -a 256 "$TEMP_FILE" | cut -d' ' -f1)
else
    warning "⚠ No se pudo verificar el checksum (sha256sum o shasum no encontrado)"
    CHECKSUM="N/A"
fi
info "SHA256: $CHECKSUM"

# Instalar binario
EXE_PATH="$INSTALL_DIR/nvm"
info ""
info "Instalando binario..."

# Hacer backup si existe
if [ -f "$EXE_PATH" ]; then
    BACKUP_PATH="$INSTALL_DIR/nvm.bak"
    mv "$EXE_PATH" "$BACKUP_PATH"
    warning "⚠ Backup creado: nvm.bak"
fi

# Copiar nuevo binario
mv "$TEMP_FILE" "$EXE_PATH"
chmod +x "$EXE_PATH"
success "✓ Binario instalado en: $EXE_PATH"

# Verificar que funciona
info ""
info "Verificando instalación..."
if NVM_VERSION=$("$EXE_PATH" --version 2>&1); then
    success "✓ nvm instalado correctamente: $NVM_VERSION"
else
    warning "⚠ No se pudo verificar la instalación automáticamente"
fi

# Configurar variables de entorno
info ""
info "Configurando variables de entorno..."

# Detectar shell
SHELL_RC=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
else
    SHELL_RC="$HOME/.profile"
fi

# 1. Configurar NVM_HOME (home directory)
NVM_HOME_DIR="$HOME/.nvm"
if grep -q "export NVM_HOME=" "$SHELL_RC" 2>/dev/null; then
    info "Variable NVM_HOME ya configurada en $SHELL_RC"
else
    echo "" >> "$SHELL_RC"
    echo "# nvm-rs configuration (homologated)" >> "$SHELL_RC"
    echo "export NVM_HOME=\"$NVM_HOME_DIR\"" >> "$SHELL_RC"
    success "✓ Variable NVM_HOME agregada a $SHELL_RC"
fi

# 2. Configurar NVM_BIN
if ! grep -q "export NVM_BIN=" "$SHELL_RC" 2>/dev/null; then
    echo "export NVM_BIN=\"\$NVM_HOME/bin\"" >> "$SHELL_RC"
    success "✓ Variable NVM_BIN agregada a $SHELL_RC"
fi

# 3. Configurar NVM_NODE
if ! grep -q "export NVM_NODE=" "$SHELL_RC" 2>/dev/null; then
    echo "export NVM_NODE=\"\$NVM_HOME/current/bin\"" >> "$SHELL_RC"
    success "✓ Variable NVM_NODE agregada a $SHELL_RC"
fi

# Verificar PATH
info ""
info "Verificando configuración del PATH..."
CURRENT_BIN="$NVM_HOME_DIR/current/bin"
NVM_BIN_PATH="$NVM_HOME_DIR/bin"
PATH_CONFIGURED=false

if echo "$PATH" | grep -q "$NVM_BIN_PATH" && echo "$PATH" | grep -q "$CURRENT_BIN"; then
    success "✓ NVM_BIN y NVM_NODE ya están configurados en el PATH"
    PATH_CONFIGURED=true
else
    warning "⚠ Falta configurar el PATH completo"

    # Ofrecer agregar automáticamente
    if [ -t 0 ]; then
        echo ""
        read -p "¿Desea agregar al PATH automáticamente? (s/N): " -r
        echo ""
        if [[ $REPLY =~ ^[SsYy]$ ]]; then
            # Verificar si ya está configurado
            NEEDS_UPDATE=false
            if ! grep -q "export PATH=.*\$NVM_HOME/bin" "$SHELL_RC" 2>/dev/null; then
                NEEDS_UPDATE=true
            fi
            if ! grep -q "export PATH=.*\$NVM_HOME/current/bin" "$SHELL_RC" 2>/dev/null; then
                NEEDS_UPDATE=true
            fi

            if [ "$NEEDS_UPDATE" = true ]; then
                echo "export PATH=\"\$NVM_HOME/bin:\$NVM_HOME/current/bin:\$PATH\"" >> "$SHELL_RC"
                success "✓ PATH actualizado en $SHELL_RC"
                warning "⚠ Reinicie su terminal o ejecute: source $SHELL_RC"
            else
                info "PATH ya configurado en $SHELL_RC"
            fi
        else
            echo ""
            info "Para agregar al PATH manualmente:"
            info "Agregar al final de $SHELL_RC:"
            echo ""
            echo "  export PATH=\"\$NVM_HOME/bin:\$NVM_HOME/current/bin:\$PATH\""
            echo ""
            info "Luego, recargar la configuración:"
            echo "  source $SHELL_RC"
        fi
    else
        echo ""
        info "Para agregar al PATH:"
        info "Agregar al final de $SHELL_RC:"
        echo ""
        echo "  export PATH=\"\$NVM_HOME/bin:\$NVM_HOME/current/bin:\$PATH\""
        echo ""
        info "Luego, recargar la configuración:"
        echo "  source $SHELL_RC"
    fi
fi

echo ""
success "=== ✓ Instalación completada ==="
echo ""
info "Para comenzar a usar nvm:"
info "  nvm --help             # Ver ayuda"
info "  nvm ls-remote          # Listar versiones disponibles de Node.js"
info "  nvm install latest     # Instalar última versión de Node.js"
info "  nvm use latest         # Usar última versión instalada"
echo ""
info "Para actualizar nvm en el futuro:"
info "  nvm update-self        # Actualizar a la última versión"
echo ""
success "¡Gracias por usar nvm-rs!"
