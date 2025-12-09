# Quick Start: Release nvm-rs v0.5.0

## ✅ Estado Actual

- [x] Scripts de build creados
- [x] Script de validación creado
- [x] Script de publicación creado
- [x] Instalador actualizado con detección flexible de assets
- [x] Build de prueba exitoso: `nvm-v0.5.0-windows-x64.exe` (4.23 MB)
- [x] Validación de prueba: ✅ PASS

## 🚀 Próximos Pasos

### 1. Publicar Release Actual

```powershell
# En d:\Elementum\repo\nvm-rs

# Publicar como draft (para revisar primero)
.\scripts\publish-release.ps1 -Version v0.5.0 -Draft

# Revisar en GitHub
gh release view v0.5.0

# Si todo está bien, publicar
gh release edit v0.5.0 --draft=false
```

### 2. Probar Instalador

```powershell
# Eliminar instalación actual (opcional)
nvm uninstall-self -yes

# Reinstalar desde GitHub
iwr -useb https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.0/install.ps1 | iex

# O si quieres probar con el instalador local primero
.\scripts\install.ps1 -Version v0.5.0
```

### 3. Verificar Funcionamiento

```powershell
# Verificar versión
nvm --version

# Verificar que ls-remote funciona
nvm ls-remote

# Instalar una versión de Node.js
nvm install lts

# Usar la versión instalada
nvm use lts

# Verificar Node.js
node --version
npm --version
```

## 📦 Assets que se Subirán

```
release-builds/
├── nvm-v0.5.0-windows-x64.exe    (4.23 MB)
├── CHECKSUMS.sha256               (checksums SHA256)
└── manifest.json                  (metadatos del build)
```

## 🎯 Para Futuras Releases Multi-Plataforma

### En Máquina Windows

```powershell
# Build Windows
.\scripts\build-releases.ps1
```

### En Máquina Linux

```bash
# Build Linux (todas las variantes)
./scripts/build.sh --target linux-gnu-x64
./scripts/build.sh --target linux-musl-x64
./scripts/build.sh --target linux-gnu-arm64
./scripts/build.sh --target linux-musl-arm64
```

### En Máquina macOS

```bash
# Build macOS
./scripts/build.sh --target macos-x64
./scripts/build.sh --target macos-arm64
```

### Combinar y Publicar

```powershell
# 1. Copiar todos los binarios a release-builds/

# 2. Validar
.\scripts\validate-release.ps1 -Strict

# 3. Publicar todo
.\scripts\publish-release.ps1 -Version v0.5.0
```

## 📝 Notas Importantes

### Sobre el Instalador

El instalador (`scripts/install.ps1`) ahora:

1. Busca assets con múltiples patrones:
   - `nvm-vX.Y.Z-self-update-windows-x64.exe`
   - `nvm-vX.Y.Z-windows-x64.exe`
   - `nvm-X.Y.Z-windows-x64.exe`
   - `*windows-x64.exe` (fallback)
   - `nvm.exe` (fallback final)

2. Es compatible con releases parciales (solo Windows)

3. Muestra assets disponibles si no encuentra uno compatible

### Sobre el Script de Build

El script `build-releases.ps1`:

- En Windows: Solo compila targets de Windows por defecto
- Usa nombres compatibles: `nvm-vX.Y.Z-OS-ARCH[.ext]`
- Genera checksums SHA256 automáticamente
- Crea manifest.json con metadatos
- Valida que los binarios no estén vacíos

### Sobre el Script de Validación

El script `validate-release.ps1`:

- Verifica nombres de archivo
- Valida checksums
- Comprueba tamaños de archivo
- Revisa plataformas requeridas
- Valida manifest JSON
- Modo estricto opcional (`-Strict`)

## 🔧 Comandos Útiles

```powershell
# Build local rápido
cargo build --release

# Build con validación
make release

# Solo crear tag (sin publicar)
.\scripts\publish-release.ps1 -Version v0.5.0 -TagOnly

# Ver release en GitHub
gh release view v0.5.0

# Descargar asset específico
gh release download v0.5.0 -p "nvm-v0.5.0-windows-x64.exe"

# Eliminar release (si algo salió mal)
gh release delete v0.5.0 --yes
git tag -d v0.5.0
git push origin :refs/tags/v0.5.0
```

## 📚 Documentación Disponible

- `scripts/README.md` - Guía rápida de scripts
- `scripts/BUILD_GUIDE.md` - Guía detallada completa
- `Makefile` - Comandos make disponibles
- Este archivo - Quick start

## 🎉 Release Checklist

Antes de publicar:

- [x] Código compilado correctamente
- [x] Scripts de build funcionando
- [x] Validación pasando
- [ ] Release notes escritas (opcional)
- [ ] Changelog actualizado (opcional)
- [ ] Tag creado
- [ ] Assets subidos a GitHub
- [ ] Instalador probado
- [ ] Documentación actualizada
- [ ] Anuncio preparado

## 🚨 En Caso de Problemas

### Si el instalador no encuentra el asset

1. Verificar que el asset está en la release:
   ```powershell
   gh release view v0.5.0 --json assets
   ```

2. Verificar el nombre del asset:
   ```powershell
   ls release-builds/
   ```

3. Re-subir el asset si es necesario:
   ```powershell
   gh release upload v0.5.0 release-builds/nvm-v0.5.0-windows-x64.exe
   ```

### Si el build falla

1. Limpiar:
   ```powershell
   cargo clean
   ```

2. Verificar target instalado:
   ```powershell
   rustup target list --installed
   ```

3. Reinstalar target si es necesario:
   ```powershell
   rustup target add x86_64-pc-windows-msvc
   ```

### Si la validación falla

1. Ver detalles:
   ```powershell
   .\scripts\validate-release.ps1
   ```

2. Revisar checksums:
   ```powershell
   Get-FileHash release-builds/nvm-v0.5.0-windows-x64.exe -Algorithm SHA256
   cat release-builds/CHECKSUMS.sha256
   ```

---

**Ready to Release!** 🎊

Ejecuta:
```powershell
.\scripts\publish-release.ps1 -Version v0.5.0 -Draft
```
