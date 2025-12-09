# Scripts de Compilación

Scripts para compilar nvm-rs para múltiples plataformas.

## Quick Start

### En Windows

```powershell
.\build-releases.ps1
```

### En Linux/macOS

```bash
# Instalar dependencias (primera vez)
sudo bash ./setup-linux-build-env.sh

# Compilar
bash ./build.sh
```

## Archivos

### `build-releases.ps1` (PowerShell - Recomendado en Windows)

Script principal de compilación cross-platform. Compila nvm-rs para todos los targets soportados.

**Uso:**

```powershell
.\build-releases.ps1                    # Compilar todos los targets
.\build-releases.ps1 -Target windows-x64 # Compilar solo Windows x64
.\build-releases.ps1 -Target linux-x64   # Compilar solo Linux x64
```

**Características:**

- Compila múltiples targets automáticamente
- Genera checksums SHA256
- Crea manifest.json con metadatos
- Valida compilaciones
- Salida clara y structured

**Output:** `../release-builds/`

### `build.sh` (Bash/Shell - Linux/macOS)

Script de compilación para sistemas Unix/Linux/macOS.

**Uso:**

```bash
./build.sh                    # Auto-detecta OS y compila
./build.sh linux-x64          # Especificar target
./build.sh --help             # Ver ayuda
```

**Características:**

- Auto-detección de OS y arquitectura
- Fallback a cargo si PowerShell no disponible
- Compatible con CI/CD

**Output:** `../release-builds/`

### `build.bat` (CMD.exe - Windows)

Wrapper de CMD.exe para PowerShell (compatibilidad legacy).

**Uso:**

```cmd
build.bat                     # Compilar todos
build.bat windows-x64         # Compilar solo Windows x64
```

**Nota:** Es simplemente un wrapper que llama a `build-releases.ps1`.

## Targets Soportados

| Target | Nombre Binario |
|--------|---|
| `windows-x64` | `nvm-vX.Y.Z-windows-x64.exe` |
| `windows-arm64` | `nvm-vX.Y.Z-windows-arm64.exe` |
| `linux-x64` | `nvm-vX.Y.Z-linux-x64` |
| `linux-arm64` | `nvm-vX.Y.Z-linux-arm64` |
| `macos-x64` | `nvm-vX.Y.Z-macos-x64` |
| `macos-arm64` | `nvm-vX.Y.Z-macos-arm64` |

## Artifacts Generados

Después de una compilación exitosa:

```
release-builds/
├── nvm-vX.Y.Z-PLATFORM-ARCH.exe   # Binarios compilados
├── CHECKSUMS.sha256                # Checksums SHA256
└── manifest.json                   # Metadata de compilación
```

## Ejemplos

**Compilar Windows x64:**

```powershell
cd scripts/build
.\build-releases.ps1 -Target windows-x64
```

**Compilar en Linux:**

```bash
cd scripts/build
./build.sh linux-x64
```

**Compilar todo desde Makefile:**

```bash
make build-all
```

## Troubleshooting

**Error: "cargo not found"**

- Instalar Rust: <https://rustup.rs/>
- Instalar targets: `rustup target add x86_64-pc-windows-msvc`

**Error: "Target not supported"**

- Ver lista de targets disponibles en el script
- Asegurarse de tener los targets de Rust instalados

**Compilación lenta**

- Primera compilación puede tardar 1-2 minutos
- Compilaciones subsecuentes son más rápidas
- Limpiar caché: `cargo clean` si es necesario
