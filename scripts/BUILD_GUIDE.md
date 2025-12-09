# Guía de Build y Release para nvm-rs

Este documento describe el proceso completo para compilar y publicar releases multi-plataforma de nvm-rs.

## Descripción General

nvm-rs proporciona soporte para múltiples plataformas y arquitecturas:

### Plataformas Soportadas

| Plataforma | Arquitecturas | Estado |
|-----------|---------------|---------|
| **Windows** | x64, ARM64 | ✅ Completamente soportado |
| **Linux** | x64 (glibc/musl), ARM64 (glibc/musl) | ✅ Completamente soportado |
| **macOS** | x64 (Intel), ARM64 (Apple Silicon) | ✅ Completamente soportado |

### Formato de Nombres de Binarios

Los binarios deben seguir este formato para ser reconocidos por el instalador:

```
nvm-vX.Y.Z-OS-ARCH[.ext]
```

- **X.Y.Z**: Versión (ej: 0.5.0)
- **OS**: `windows`, `linux`, `macos`
- **ARCH**: `x64`, `arm64`
- **ext**: `.exe` solo para Windows

**Ejemplos:**

- `nvm-v0.5.0-windows-x64.exe` → Windows 64-bit
- `nvm-v0.5.0-windows-arm64.exe` → Windows ARM64
- `nvm-v0.5.0-linux-x64` → Linux 64-bit
- `nvm-v0.5.0-linux-arm64` → Linux ARM64
- `nvm-v0.5.0-macos-x64` → macOS Intel
- `nvm-v0.5.0-macos-arm64` → macOS Apple Silicon

### Variantes de Build

Cada binario puede tener dos variantes:

1. **Release Normal**: `nvm-vX.Y.Z-OS-ARCH[.ext]`
2. **Con Self-Update**: `nvm-v{version}-self-update-OS-ARCH[.ext]`

## Prerequisites

Antes de iniciar, asegúrate de tener instalado:

```powershell
# Rust & Cargo
https://rustup.rs

# Rust targets cross-compilation
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
# ... más targets según necesites
```

Para compilación cruzada en Linux, instala:

```bash
# Para musl
apt-get install musl-tools

# Para ARM64
apt-get install gcc-aarch64-linux-gnu

# Para glibc ARM64
apt-get install gcc-aarch64-linux-gnu
```

## Workflow de Build

### 1. Build Local (Windows)

Para compilar solo los binarios de Windows en una máquina Windows:

```powershell
# Build standard (sin self-update)
./scripts/build-releases.ps1

# Build con self-update
./scripts/build-releases.ps1 -WithSelfUpdate

# Build específico
./scripts/build-releases.ps1 -Target windows-x64

# Build debug
./scripts/build-releases.ps1 -BuildType debug
```

**Output:**

- Los binarios se guardan en `./release-builds/`
- Se genera `CHECKSUMS.sha256` con los hashes
- Se genera `manifest.json` con metadatos

### 2. Cross-Compilation en Linux

Para compilar todos los targets en una máquina Linux:

```bash
# Compilar todos los targets
./scripts/build-releases.ps1

# Compilar específico
./scripts/build-releases.ps1 -Target linux-gnu-x64
```

### 3. Validar Build

Antes de publicar, valida los artifacts:

```powershell
# Validación básica
./scripts/validate-release.ps1

# Validación estricta (falla con warnings)
./scripts/validate-release.ps1 -Strict

# Validación personalizada
./scripts/validate-release.ps1 -ReleaseDir ./release-v0.5.0
```

**Verifica:**

- ✅ Nombres de archivo correctos
- ✅ Integridad de checksums
- ✅ Binarios no vacíos
- ✅ Plataformas necesarias presentes
- ✅ Manifest válido

## Workflow de Release

### 1. Preparar Release

```powershell
# Crear tag de Git
./scripts/publish-release.ps1 -Version v0.5.0 -TagOnly

# O crear en GitHub directamente (como draft)
./scripts/publish-release.ps1 -Version v0.5.0 -Draft
```

### 2. Publicar en GitHub

```powershell
# Publicar como release (no draft)
./scripts/publish-release.ps1 -Version v0.5.0

# Publicar como pre-release
./scripts/publish-release.ps1 -Version v0.5.0 -PreRelease

# Publicar actualizando instaladores
./scripts/publish-release.ps1 -Version v0.5.0 -UpdateInstaller
```

### 3. Verificar Release

```powershell
# Ver información del release
gh release view v0.5.0

# Listar assets
gh release view v0.5.0 --json assets

# Descargar un asset
gh release download v0.5.0 -p "nvm-v0.5.0-windows-x64.exe"
```

## Flujo Completo (Paso a Paso)

### Escenario 1: Release en Windows

```powershell
# 1. Actualizar versión en Cargo.toml si es necesario
# 2. Compilar binarios
./scripts/build-releases.ps1

# 3. Validar
./scripts/validate-release.ps1 -Strict

# 4. Crear tag y publicar
./scripts/publish-release.ps1 -Version v0.5.0

# 5. Verificar en GitHub
gh release view v0.5.0
```

### Escenario 2: Release Multi-Plataforma

**En máquina Windows:**

```powershell
./scripts/build-releases.ps1 -Target windows-x64
./scripts/build-releases.ps1 -Target windows-arm64
# Guardar artifacts
```

**En máquina Linux:**

```bash
./scripts/build-releases.ps1 # Compila linux-gnu-x64, linux-musl-x64, etc.
# Guardar artifacts
```

**En máquina macOS:**

```bash
./scripts/build-releases.ps1 # Compila macos-x64, macos-arm64
# Guardar artifacts
```

**Combinar y publicar:**

```powershell
# Copiar todos los artifacts a ./release-builds/
# Validar
./scripts/validate-release.ps1 -Strict

# Publicar
./scripts/publish-release.ps1 -Version v0.5.0
```

## Estructura de Directorios de Build

```
nvm-rs/
├── scripts/
│   ├── build-releases.ps1      # Script principal de build
│   ├── publish-release.ps1     # Script de publicación
│   ├── validate-release.ps1    # Script de validación
│   ├── install.ps1             # Instalador para Windows
│   └── install.sh              # Instalador para Unix
├── release-builds/             # Directorio de output
│   ├── nvm-v0.5.0-windows-x64.exe
│   ├── nvm-v0.5.0-linux-x64
│   ├── nvm-v0.5.0-macos-arm64
│   ├── CHECKSUMS.sha256        # Hashes SHA256
│   └── manifest.json           # Metadatos
└── src/                        # Código fuente
    └── main.rs
```

## Instalador

### PowerShell (Windows)

El instalador `scripts/install.ps1` busca automáticamente el binario correcto:

```powershell
# Instalar última versión
iwr -useb https://github.com/FreddyCamposeco/nvm-rs/releases/download/latest/install.ps1 | iex

# Instalar versión específica
iwr -useb https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.0/install.ps1 | iex

# Instalar con self-update
iwr -useb https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.0/install.ps1 -OutFile install.ps1
./install.ps1 -WithSelfUpdate
```

**Lógica de búsqueda de assets:**

1. Busca `nvm-vX.Y.Z-self-update-windows-ARCH.exe` (si -WithSelfUpdate)
2. Busca `nvm-vX.Y.Z-windows-ARCH.exe`
3. Busca `nvm-X.Y.Z-windows-ARCH.exe`
4. Busca cualquier `*windows-ARCH.exe`
5. Fallback a `nvm.exe`

### Shell Script (Linux/macOS)

```bash
# Instalar última versión
curl -fsSL https://github.com/FreddyCamposeco/nvm-rs/releases/download/latest/install.sh | bash

# Instalar versión específica
curl -fsSL https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.0/install.sh | bash

# Instalar con self-update
curl -fsSL https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.0/install.sh -o install.sh
chmod +x install.sh
./install.sh --with-self-update
```

## Automatización con GitHub Actions

Se puede automatizar el build con GitHub Actions:

```yaml
name: Cross-Platform Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: windows-latest
            target: windows-x64
          - os: ubuntu-latest
            target: linux-gnu-x64
          - os: macos-latest
            target: macos-x64

    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}

      - name: Build
        run: ./scripts/build-releases.ps1 -Target ${{ matrix.target }}

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: artifacts-${{ matrix.target }}
          path: release-builds/
```

## Solución de Problemas

### Error: Target not installed

```powershell
rustup target add x86_64-unknown-linux-gnu
```

### Error: Cross-compile linker not found

Instala las herramientas de compilación cruzada necesarias:

```bash
# Linux -> Windows
apt-get install mingw-w64

# Linux -> macOS (experimental)
# Requiere Xcode build tools en macOS
```

### Binario muy pequeño

Verifica que la compilación en release está activada:

```powershell
./scripts/build-releases.ps1 -BuildType release
```

### Asset no encontrado en instalador

1. Verifica el nombre del asset sigue el formato correcto
2. Carga el asset en la release de GitHub
3. Ejecuta validación: `./scripts/validate-release.ps1`

## Referencias

- [Rust Targets](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [Cargo Cross Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
- [GitHub CLI](https://cli.github.com/)
- [GitHub Releases API](https://docs.github.com/en/rest/releases)
