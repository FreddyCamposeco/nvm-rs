# Scripts de Release y Validación

Scripts para publicar releases y validar artifacts en GitHub.

## Archivos

### `publish-release.ps1`

Script para crear y publicar releases en GitHub. Automatiza todo el proceso de creación de tags, releases y carga de artifacts.

**Uso:**

```powershell
.\publish-release.ps1 -Version v0.5.0                  # Release normal
.\publish-release.ps1 -Version v0.5.0 -Draft          # Draft (no publicado aún)
.\publish-release.ps1 -Version v0.5.0 -PreRelease     # Pre-release (beta/rc)
```

**Características:**
- Crea git tag automáticamente
- Genera notas de release
- Carga todos los artifacts
- Soporta draft y pre-release
- Actualiza installer scripts

**Prerequisitos:**
- Git instalado
- GitHub CLI (`gh`) autenticado
- Acceso push al repositorio
- Artifacts compilados en `../release-builds/`

**Output:** Release publicada en GitHub

### `validate-release.ps1`

Script para validar que los artifacts están listos para publicación. Verifica formato de nombres, checksums, integridad del manifest, etc.

**Uso:**

```powershell
.\validate-release.ps1                   # Validación básica
.\validate-release.ps1 -Strict           # Validación estricta
.\validate-release.ps1 -Verbose          # Con detalles
```

**Validaciones:**
- Nombres de archivos (formato correcto: `nvm-vX.Y.Z-PLATFORM-ARCH`)
- Checksums SHA256 válidos
- Manifest JSON bien formado
- Cobertura de plataformas
- Tamaño de binarios razonable
- Permisos de archivo correctos

**Niveles de Validación:**

| Nivel | Descripción |
|-------|---|
| **Basic** | Verifica formato y checksums |
| **Strict** | Además valida cobertura y metadata |

**Estados de Salida:**

```
0 (✓ PASS)         - Todo válido
1 (⚠ WARNING)      - Tiene advertencias pero es usable
2 (✗ ERROR)        - Fallos críticos, no publicar
```

## Workflow de Release

### Paso 1: Compilar

```powershell
cd scripts/build
.\build-releases.ps1 -Target windows-x64
```

### Paso 2: Validar

```powershell
cd scripts/release
.\validate-release.ps1
```

### Paso 3: Revisar

```powershell
ls ../release-builds/
# Verificar:
# - Archivos presentes
# - Nombres correctos
# - Checksums válidos
```

### Paso 4: Publicar

```powershell
.\publish-release.ps1 -Version v0.5.0 -Draft  # Primero como draft
# Revisar en GitHub
.\publish-release.ps1 -Version v0.5.0          # Publicar como release
```

## Estructura de Artifacts

### Directorio: `../release-builds/`

```
release-builds/
├── nvm-v0.5.0-windows-x64.exe      # Binario Windows x64
├── nvm-v0.5.0-linux-x64            # Binario Linux x64
├── nvm-v0.5.0-macos-arm64          # Binario macOS ARM64
├── CHECKSUMS.sha256                # Todos los checksums
└── manifest.json                   # Metadata de compilación
```

### Formato CHECKSUMS.sha256

```
6314B466DC560E825544438ACD3676638F80FCA01177D94F7589FAFEC5452572  nvm-v0.5.0-windows-x64.exe
A1B2C3D4E5F6...                                                      nvm-v0.5.0-linux-x64
...
```

### Formato manifest.json

```json
{
  "version": "0.5.0",
  "buildDate": "2025-12-08T10:30:00Z",
  "targets": {
    "windows-x64": {
      "binary": "nvm-v0.5.0-windows-x64.exe",
      "size": 4337152,
      "checksum": "6314B466..."
    }
  }
}
```

## Ejemplos Completos

### Release Estable

```powershell
# 1. Compilar
.\build-releases.ps1 -Target windows-x64

# 2. Validar
.\validate-release.ps1 -Strict

# 3. Publicar como draft primero
.\publish-release.ps1 -Version v0.5.0 -Draft

# 4. Revisar en https://github.com/FreddyCamposeco/nvm-rs/releases

# 5. Publicar como release
.\publish-release.ps1 -Version v0.5.0
```

### Release Beta

```powershell
.\publish-release.ps1 -Version v0.5.0-beta -PreRelease
```

### Validación Rápida

```powershell
.\validate-release.ps1 -Verbose
```

## Troubleshooting

**Error: "GitHub CLI not authenticated"**
```powershell
gh auth login
gh auth status  # Verificar
```

**Error: "Assets not found"**
- Asegurar que los binarios están compilados
- Verificar ruta: `../release-builds/`
- Verificar nombres de archivo

**Error: "Tag already exists"**
- El tag ya fue creado previamente
- Usar `git tag -d vX.Y.Z` para eliminar localmente
- Usar `git push origin --delete vX.Y.Z` para eliminar en remote

**Validación falla pero quiero publicar**
- Revisar advertencias: pueden ser no críticas
- Si hay errores críticos: compilar nuevamente
- Usar `-Draft` primero para revisar

## Links Útiles

- **GitHub CLI Docs**: https://cli.github.com/manual
- **GitHub Releases API**: https://docs.github.com/rest/releases
- **Build Scripts**: Ver `../build/README.md`
- **Quick Release**: Ver `../../docs/QUICK_RELEASE.md`
