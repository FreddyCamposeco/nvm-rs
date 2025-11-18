#!/usr/bin/env bash
# Script de desinstalación de nvm-rs para Linux/macOS

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
INSTALL_DIR="${NVM_INSTALL_DIR:-$HOME/.local/bin}"
REMOVE_NODE_VERSIONS="${NVM_REMOVE_VERSIONS:-false}"

info "=== Desinstalador de nvm-rs para Linux/macOS ==="
echo ""

EXE_PATH="$INSTALL_DIR/nvm"

# Verificar si nvm está instalado
if [ ! -f "$EXE_PATH" ]; then
    error "nvm no encontrado en: $EXE_PATH"
fi

info "Ubicación de nvm: $EXE_PATH"

# Confirmación
if [ -t 0 ]; then
    echo ""
    warning "Esta acción eliminará:"
    warning "  - El binario de nvm: $EXE_PATH"
    warning "  - Las configuraciones de PATH y NVM_DIR de los archivos shell"
    if [ "$REMOVE_NODE_VERSIONS" = "true" ]; then
        warning "  - TODAS las versiones de Node.js instaladas"
    else
        echo ""
        info "Las versiones de Node.js instaladas NO serán eliminadas."
        info "Establece NVM_REMOVE_VERSIONS=true para eliminarlas también."
    fi
    echo ""
    read -p "¿Desea continuar? (s/N): " -r
    echo ""
    if [[ ! $REPLY =~ ^[SsYy]$ ]]; then
        info "Desinstalación cancelada"
        exit 0
    fi
fi

info ""
info "Desinstalando nvm-rs..."

# Detectar archivos de configuración shell
SHELL_CONFIGS=()
[ -f "$HOME/.bashrc" ] && SHELL_CONFIGS+=("$HOME/.bashrc")
[ -f "$HOME/.zshrc" ] && SHELL_CONFIGS+=("$HOME/.zshrc")
[ -f "$HOME/.profile" ] && SHELL_CONFIGS+=("$HOME/.profile")
[ -f "$HOME/.bash_profile" ] && SHELL_CONFIGS+=("$HOME/.bash_profile")

# 1. Eliminar configuraciones de los archivos shell
info ""
info "Eliminando configuraciones de archivos shell..."
for config in "${SHELL_CONFIGS[@]}"; do
    if [ -f "$config" ]; then
        # Crear backup
        cp "$config" "${config}.bak"
        
        # Eliminar líneas de nvm-rs
        sed -i.tmp '/# nvm-rs configuration/d' "$config" 2>/dev/null || sed -i '' '/# nvm-rs configuration/d' "$config" 2>/dev/null || true
        sed -i.tmp '/# nvm-rs/d' "$config" 2>/dev/null || sed -i '' '/# nvm-rs/d' "$config" 2>/dev/null || true
        sed -i.tmp "\|export NVM_DIR=|d" "$config" 2>/dev/null || sed -i '' "\|export NVM_DIR=|d" "$config" 2>/dev/null || true
        sed -i.tmp "\|export PATH=\"$INSTALL_DIR:\$PATH\"|d" "$config" 2>/dev/null || sed -i '' "\|export PATH=\"$INSTALL_DIR:\$PATH\"|d" "$config" 2>/dev/null || true
        sed -i.tmp "\|export PATH='$INSTALL_DIR:\$PATH'|d" "$config" 2>/dev/null || sed -i '' "\|export PATH='$INSTALL_DIR:\$PATH'|d" "$config" 2>/dev/null || true
        
        # Limpiar archivos temporales
        rm -f "${config}.tmp"
        
        success "✓ Configuraciones eliminadas de $(basename $config)"
    fi
done

# 2. Eliminar binario
info ""
info "Eliminando binario..."
rm -f "$EXE_PATH"
success "✓ Binario eliminado"

# Eliminar backup si existe
if [ -f "$INSTALL_DIR/nvm.bak" ]; then
    rm -f "$INSTALL_DIR/nvm.bak"
    success "✓ Backup eliminado"
fi

# Eliminar directorio si está vacío
if [ -d "$INSTALL_DIR" ]; then
    if [ -z "$(ls -A $INSTALL_DIR)" ]; then
        rmdir "$INSTALL_DIR"
        success "✓ Directorio de instalación eliminado"
    fi
fi

# 3. Eliminar versiones de Node.js (opcional)
if [ "$REMOVE_NODE_VERSIONS" = "true" ]; then
    info ""
    info "Eliminando versiones de Node.js..."
    
    NVM_DATA_DIR="$HOME/.nvm"
    if [ -d "$NVM_DATA_DIR" ]; then
        rm -rf "$NVM_DATA_DIR"
        success "✓ Versiones de Node.js eliminadas"
    else
        info "No se encontraron versiones de Node.js instaladas"
    fi
fi

echo ""
success "=== ✓ Desinstalación completada ==="
echo ""
info "nvm-rs ha sido desinstalado del sistema."
warning "Reinicie su terminal para que los cambios surtan efecto."

if [ "$REMOVE_NODE_VERSIONS" != "true" ]; then
    echo ""
    info "Nota: Las versiones de Node.js instaladas permanecen en:"
    info "  $HOME/.nvm"
    echo ""
    info "Para eliminarlas manualmente, ejecute:"
    info "  rm -rf $HOME/.nvm"
fi

echo ""
info "Los archivos de configuración shell tienen copias de respaldo:"
for config in "${SHELL_CONFIGS[@]}"; do
    if [ -f "${config}.bak" ]; then
        info "  ${config}.bak"
    fi
done

echo ""
info "¡Gracias por usar nvm-rs!"
