# Scripts de Build y Release para nvm-rs

## 📋 Resumen

Se han creado scripts completos para compilar y publicar releases multi-plataforma de nvm-rs con nombres de binarios compatibles con el instalador.

## 🎯 Problema Resuelto

**Error anterior:**

```
❯ nvm install-self
Error: Asset nvm-v0.5.0-windows-x64.exe not found for your platform
```

**Causa:** Los binarios no seguían el formato esperado por el instalador.

**Solución:** Scripts automatizados que:

1. Compilan para múltiples plataformas
2. Generan nombres correctos (`nvm-vX.Y.Z-OS-ARCH[.ext]`)
3. Validan checksums
4. Publican en GitHub Releases

## 📦 Archivos Creados

### Scripts Principales

| Script | Descripción | Uso |
|--------|-------------|-----|
| `scripts/build-releases.ps1` | Build multi-plataforma (PowerShell) | `.\scripts\build-releases.ps1` |
| `scripts/build.sh` | Build para Unix/Linux/macOS | `./scripts/build.sh` |
| `scripts/build.bat` | Wrapper para CMD.exe | `build.bat` |
| `scripts/publish-release.ps1` | Publicar en GitHub | `.\scripts\publish-release.ps1` |
| `scripts/validate-release.ps1` | Validar artifacts | `.\scripts\validate-release.ps1` |
| `Makefile` | Comandos de build simplificados | `make build-all` |

### Documentación

- `scripts/BUILD_GUIDE.md` - Guía completa de build y release
- `scripts/README.md` - Este archivo

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
