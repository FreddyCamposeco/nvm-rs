# Scripts de nvm-rs

Colección organizada de scripts para compilar, instalar y publicar nvm-rs.

## 📁 Estructura

```
scripts/
├── build/                  # Scripts de compilación
│   ├── build-releases.ps1  # Build multi-plataforma (PowerShell)
│   ├── build.sh            # Build para Unix/Linux/macOS
│   ├── build.bat           # Wrapper para CMD.exe
│   └── README.md           # Documentación de build
├── install/                # Scripts de instalación
│   ├── install.ps1         # Instalador Windows (PowerShell)
│   ├── install.sh          # Instalador Unix/Linux/macOS
│   ├── uninstall.ps1       # Desinstalador Windows
│   ├── uninstall.sh        # Desinstalador Unix/Linux/macOS
│   └── README.md           # Documentación de instalación
├── release/                # Scripts de publicación y validación
│   ├── publish-release.ps1 # Publicar en GitHub
│   ├── validate-release.ps1 # Validar artifacts
│   └── README.md           # Documentación de release
├── BUILD_GUIDE.md          # Guía completa de compilación
└── README.md               # Este archivo
```

## 🎯 Categorías de Scripts

### 🔨 Compilación (`build/`)

Compila nvm-rs para múltiples plataformas y arquitecturas.

| Script | Plataforma | Uso |
|--------|-----------|-----|
| `build-releases.ps1` | Windows | `.\build\build-releases.ps1 -Target windows-x64` |
| `build.sh` | Linux/macOS | `./build/build.sh linux-x64` |
| `build.bat` | Windows (CMD) | `build.bat` |

**Ver:** `build/README.md`

### 📥 Instalación (`install/`)

Instala y desinstala nvm-rs en el sistema.

| Script | Plataforma | Uso |
|--------|-----------|-----|
| `install.ps1` | Windows | `.\install\install.ps1` |
| `install.sh` | Linux/macOS | `./install/install.sh` |
| `uninstall.ps1` | Windows | `.\install\uninstall.ps1` |
| `uninstall.sh` | Linux/macOS | `./install/uninstall.sh` |

**Ver:** `install/README.md`

### 🚀 Release (`release/`)

Publica y valida releases en GitHub.

| Script | Función | Uso |
|--------|---------|-----|
| `validate-release.ps1` | Validar artifacts | `.\release\validate-release.ps1` |
| `publish-release.ps1` | Publicar en GitHub | `.\release\publish-release.ps1 -Version v0.5.0` |

**Ver:** `release/README.md`

## 📋 Guías Rápidas

### Compilar

```powershell
# Windows x64
.\build\build-releases.ps1 -Target windows-x64

# Todos los targets
.\build\build-releases.ps1
```

### Instalar

```powershell
# Windows
.\install\install.ps1

# Linux/macOS
./install/install.sh
```

### Publicar Release

```powershell
# 1. Validar
.\release\validate-release.ps1

# 2. Publicar como draft
.\release\publish-release.ps1 -Version v0.5.0 -Draft

# 3. Publicar (después de revisar en GitHub)
.\release\publish-release.ps1 -Version v0.5.0
```

## 🚀 Uso Rápido

### Build Simple (Windows)

```powershell
# Build para Windows x64 solamente
.\scripts\build-releases.ps1 -Target windows-x64

# Build todos los targets de Windows
.\scripts\build-releases.ps1

# Build con self-update
.\scripts\build-releases.ps1 -WithSelfUpdate
```

### Build con Makefile

```bash
# Todas las opciones
make help

# Build release
make build-all

# Validar
make validate-release

# Release completo
make release
```

### Workflow Completo

```powershell
# 1. Compilar
.\scripts\build-releases.ps1

# 2. Validar
.\scripts\validate-release.ps1

# 3. Publicar
.\scripts\publish-release.ps1 -Version v0.5.0 -Draft

# 4. Verificar en GitHub
gh release view v0.5.0

# 5. Publicar (quitar draft)
gh release edit v0.5.0 --draft=false
```

## 📝 Formato de Nombres de Binarios

Los binarios DEBEN seguir este formato:

```
nvm-vX.Y.Z-OS-ARCH[.ext]
```

### Ejemplos Válidos

✅ `nvm-v0.5.0-windows-x64.exe`
✅ `nvm-v0.5.0-windows-arm64.exe`
✅ `nvm-v0.5.0-linux-x64`
✅ `nvm-v0.5.0-linux-arm64`
✅ `nvm-v0.5.0-macos-x64`
✅ `nvm-v0.5.0-macos-arm64`

### Variantes

**Con Self-Update:**

- `nvm-v0.5.0-self-update-windows-x64.exe`

**Por Distribución de Linux:**

- `nvm-v0.5.0-linux-gnu-x64` (glibc)
- `nvm-v0.5.0-linux-musl-x64` (musl)

## 🎨 Plataformas Soportadas

| Plataforma | Target Rust | Build en Windows | Estado |
|-----------|-------------|------------------|---------|
| Windows x64 | `x86_64-pc-windows-msvc` | ✅ | Soportado |
| Windows ARM64 | `aarch64-pc-windows-msvc` | ✅ | Soportado |
| Linux x64 (glibc) | `x86_64-unknown-linux-gnu` | ❌ | Requiere Linux |
| Linux x64 (musl) | `x86_64-unknown-linux-musl` | ❌ | Requiere Linux |
| Linux ARM64 (glibc) | `aarch64-unknown-linux-gnu` | ❌ | Requiere Linux |
| Linux ARM64 (musl) | `aarch64-unknown-linux-musl` | ❌ | Requiere Linux |
| macOS x64 | `x86_64-apple-darwin` | ❌ | Requiere macOS |
| macOS ARM64 | `aarch64-apple-darwin` | ❌ | Requiere macOS |

## 🔧 Instalación de Targets

```powershell
# Windows targets
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc

# Linux targets (en máquina Linux)
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-musl

# macOS targets (en máquina macOS)
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

O usar el Makefile:

```bash
make install-targets
```

## 📊 Validación

### Validación Manual

```powershell
# Validación básica
.\scripts\validate-release.ps1

# Validación estricta (falla con warnings)
.\scripts\validate-release.ps1 -Strict
```

### Qué se Valida

- ✅ Nombres de archivo correctos
- ✅ Extensiones apropiadas (.exe para Windows)
- ✅ Integridad de checksums SHA256
- ✅ Tamaños de archivo razonables (>100KB)
- ✅ Presencia de plataformas requeridas
- ✅ Validez del manifest JSON

## 📤 Publicación en GitHub

### Opción 1: Script de Publicación

```powershell
# Draft release
.\scripts\publish-release.ps1 -Version v0.5.0 -Draft

# Release público
.\scripts\publish-release.ps1 -Version v0.5.0

# Pre-release
.\scripts\publish-release.ps1 -Version v0.5.0 -PreRelease
```

### Opción 2: GitHub CLI Manual

```bash
# Crear release draft
gh release create v0.5.0 \
  --title "Release v0.5.0" \
  --draft \
  release-builds/*

# Publicar
gh release edit v0.5.0 --draft=false
```

## 🧪 Testing del Instalador

Después de publicar, prueba el instalador:

```powershell
# Windows
iwr -useb https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.0/install.ps1 | iex

# Linux/macOS
curl -fsSL https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.0/install.sh | bash
```

## 📁 Estructura de Directorios

```
nvm-rs/
├── scripts/
│   ├── build-releases.ps1       ← Build principal (PowerShell)
│   ├── build.sh                 ← Build para Unix
│   ├── build.bat                ← Wrapper para CMD
│   ├── publish-release.ps1      ← Publicar en GitHub
│   ├── validate-release.ps1     ← Validar artifacts
│   ├── install.ps1              ← Instalador Windows (actualizado)
│   ├── install.sh               ← Instalador Unix
│   └── BUILD_GUIDE.md           ← Documentación detallada
├── release-builds/              ← Output de compilación
│   ├── nvm-v0.5.0-windows-x64.exe
│   ├── CHECKSUMS.sha256
│   └── manifest.json
├── Makefile                     ← Comandos simplificados
└── Cargo.toml
```

## 🐛 Troubleshooting

### Error: Target not installed

```powershell
rustup target add x86_64-pc-windows-msvc
```

### Error: Permission denied (Linux)

```bash
chmod +x scripts/build.sh
./scripts/build.sh
```

### Asset no encontrado en GitHub

1. Verifica que el nombre del binario siga el formato correcto
2. Ejecuta validación: `.\scripts\validate-release.ps1`
3. Sube manualmente: `gh release upload v0.5.0 release-builds/*`

### Build falla con error de compilación

```powershell
# Limpiar cache
cargo clean

# Rebuild
.\scripts\build-releases.ps1 -Target windows-x64
```

## 📚 Recursos Adicionales

- [BUILD_GUIDE.md](./BUILD_GUIDE.md) - Guía detallada completa
- [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [Cargo Cross Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
- [GitHub CLI Documentation](https://cli.github.com/manual/)

## ✅ Checklist de Release

Antes de publicar una nueva versión:

- [ ] Actualizar `version` en `Cargo.toml`
- [ ] Actualizar `CHANGELOG.md`
- [ ] Ejecutar `cargo test`
- [ ] Build: `.\scripts\build-releases.ps1`
- [ ] Validar: `.\scripts\validate-release.ps1 -Strict`
- [ ] Crear release notes en `RELEASE_NOTES_vX.Y.Z.md`
- [ ] Publicar: `.\scripts\publish-release.ps1 -Version vX.Y.Z -Draft`
- [ ] Revisar en GitHub
- [ ] Publicar: `gh release edit vX.Y.Z --draft=false`
- [ ] Testar instalador
- [ ] Anunciar release

## 📞 Soporte

Si encuentras problemas:

1. Revisa la documentación en `BUILD_GUIDE.md`
2. Ejecuta `.\scripts\validate-release.ps1` para diagnosticar
3. Verifica logs de compilación
4. Abre un issue en GitHub con:
   - Output completo del comando
   - Sistema operativo y arquitectura
   - Versión de Rust: `rustc --version`
   - Targets instalados: `rustup target list --installed`

---

**Creado:** Diciembre 2025
**Versión:** 1.0.0
**Autor:** nvm-rs team
