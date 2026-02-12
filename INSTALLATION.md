# Guía de Instalación y Actualización de nvm-rs

Esta guía detalla todos los métodos disponibles para instalar, actualizar y desinstalar nvm-rs desde GitHub releases.

## 📥 Instalación

### Método 1: Script Automático (Recomendado)

El método más sencillo es usar los scripts de instalación automática.

#### Windows (PowerShell)

```powershell
# Instalación básica
iwr -useb https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.ps1 | iex

# Instalación con opciones
$env:NVM_VERSION="v0.6.1"                      # Versión específica (opcional)
$env:NVM_INSTALL_DIR="C:\nvm"                  # Directorio personalizado (opcional)
$env:NVM_WITH_SELF_UPDATE="true"               # Con capacidad de auto-actualización (opcional)
iwr -useb https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.ps1 | iex
```

#### Linux / macOS (Bash)

```bash
# Instalación básica
curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.sh | bash

# Instalación con opciones
export NVM_VERSION="v0.6.1"                     # Versión específica (opcional)
export NVM_INSTALL_DIR="$HOME/.local/bin"       # Directorio personalizado (opcional)
export NVM_WITH_SELF_UPDATE="true"              # Con capacidad de auto-actualización (opcional)
curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.sh | bash
```

**Características del script:**

- ✅ Detecta automáticamente tu sistema operativo y arquitectura
- ✅ Descarga la versión correcta desde GitHub Releases
- ✅ Verifica la integridad del archivo con checksum SHA256
- ✅ Instala el binario en la ubicación apropiada
- ✅ Ofrece configurar el PATH automáticamente
- ✅ Hace backup de versiones anteriores

### Método 2: Comandos Integrados de nvm

Si ya tienes nvm instalado, puedes usar los comandos integrados:

```bash
# Instalar última versión
nvm install-self

# Instalar versión específica
nvm install-self --version v0.6.1
nvm install-self -v v0.6.1

# Instalar con capacidad de auto-actualización
nvm install-self --with-self-update

# Instalar en directorio personalizado
nvm install-self --dir /usr/local/bin
nvm install-self -d C:\nvm
```

### Método 3: Descarga Manual

#### Windows

1. **Descargar el binario:**
   - Ve a [GitHub Releases](https://github.com/FreddyCamposeco/nvm-rs/releases/latest)
   - Descarga `nvm-v0.6.1-windows-x64.exe` (o la versión con `-self-update` si deseas esa funcionalidad)

2. **Verificar integridad (recomendado):**

   ```powershell
   Get-FileHash -Path "nvm-v0.6.1-windows-x64.exe" -Algorithm SHA256
   ```

3. **Instalar:**

   ```powershell
   # Renombrar
   Rename-Item "nvm-v0.6.1-windows-x64.exe" "nvm.exe"

   # Mover a ubicación deseada
   New-Item -ItemType Directory -Path "$env:LOCALAPPDATA\Programs\nvm" -Force
   Move-Item "nvm.exe" "$env:LOCALAPPDATA\Programs\nvm\"

   # Agregar al PATH
   $env:PATH += ";$env:LOCALAPPDATA\Programs\nvm"
   ```

4. **Verificar:**

   ```powershell
   nvm --version
   ```

#### Linux / macOS

1. **Descargar el binario:**

   ```bash
   # Linux x64
   wget https://github.com/FreddyCamposeco/nvm-rs/releases/latest/download/nvm-v0.6.1-linux-x64

   # macOS x64
   wget https://github.com/FreddyCamposeco/nvm-rs/releases/latest/download/nvm-v0.6.1-macos-x64

   # Linux ARM64
   wget https://github.com/FreddyCamposeco/nvm-rs/releases/latest/download/nvm-v0.6.1-linux-arm64
   ```

2. **Verificar integridad:**

   ```bash
   sha256sum nvm-v0.6.1-linux-x64
   ```

3. **Instalar:**

   ```bash
   # Hacer ejecutable
   chmod +x nvm-v0.6.1-linux-x64

   # Mover a ubicación deseada
   mkdir -p ~/.local/bin
   mv nvm-v0.6.1-linux-x64 ~/.local/bin/nvm

   # Agregar al PATH (si no está)
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

4. **Verificar:**

   ```bash
   nvm --version
   ```

### Método 4: Compilar desde Código Fuente

```bash
# Clonar repositorio
git clone https://github.com/FreddyCamposeco/nvm-rs.git
cd nvm-rs

# Compilar versión estándar
cargo build --release

# O compilar con auto-actualización
cargo build --release --features self-update

# Instalar
sudo cp target/release/nvm /usr/local/bin/
# O en Windows
copy target\release\nvm.exe C:\nvm\
```

## 🔄 Actualización

### Método 1: Comando update-self

La forma más sencilla de actualizar:

```bash
# Actualizar a la última versión
nvm update-self

# Actualizar a versión específica
nvm update-self --version v0.6.1
nvm update-self -v v0.6.1

# Actualizar con capacidad de auto-actualización
nvm update-self --with-self-update
```

### Método 2: Comando self-update (Feature Opcional)

Si compilaste con `--features self-update`:

```bash
nvm self-update
```

### Método 3: Reinstalar

Simplemente ejecuta el script de instalación nuevamente, automáticamente hará backup de la versión anterior:

```powershell
# Windows
iwr -useb https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.ps1 | iex
```

```bash
# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install.sh | bash
```

## 🗑️ Desinstalación

### Método 1: Comando uninstall-self

```bash
# Desinstalar con confirmación
nvm uninstall-self

# Desinstalar sin confirmación
nvm uninstall-self --yes
nvm uninstall-self -y

# Desinstalar desde directorio específico
nvm uninstall-self --dir /usr/local/bin
```

**Nota:** Este comando solo elimina el binario de nvm, no las versiones de Node.js instaladas.

### Método 2: Manual

#### Windows

```powershell
# Eliminar binario
Remove-Item "$env:LOCALAPPDATA\Programs\nvm\nvm.exe"

# Eliminar del PATH (en Variables de Entorno del Sistema)

# Opcionalmente, eliminar versiones de Node.js
Remove-Item -Recurse "$env:USERPROFILE\.nvm"
```

#### Linux / macOS

```bash
# Eliminar binario
rm ~/.local/bin/nvm

# Eliminar del PATH (editar ~/.bashrc o ~/.zshrc y eliminar la línea)

# Opcionalmente, eliminar versiones de Node.js
rm -rf ~/.nvm
```

## 🔧 Configuración del PATH

### Windows

**Método 1: PowerShell (temporal)**

```powershell
$env:PATH += ";C:\nvm"
```

**Método 2: Variables de Entorno (permanente)**

1. Buscar "Variables de entorno" en el menú Inicio
2. Editar la variable PATH del usuario
3. Agregar `C:\nvm` (o tu directorio de instalación)

**Método 3: PowerShell como Administrador (permanente)**

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\nvm", "User")
```

### Linux / macOS

Agregar al final de `~/.bashrc` o `~/.zshrc`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Luego recargar:

```bash
source ~/.bashrc  # o source ~/.zshrc
```

## 📋 Plataformas Soportadas

| Sistema Operativo | Arquitectura | Asset Name Pattern |
|-------------------|--------------|-------------------|
| Windows | x64 | `nvm-vX.X.X-windows-x64.exe` |
| Windows | x86 | `nvm-vX.X.X-windows-x86.exe` |
| Linux | x64 | `nvm-vX.X.X-linux-x64` |
| Linux | ARM64 | `nvm-vX.X.X-linux-arm64` |
| macOS | x64 | `nvm-vX.X.X-macos-x64` |
| macOS | ARM64 (M1/M2) | `nvm-vX.X.X-macos-arm64` |

Todos los assets están disponibles con o sin el sufijo `-self-update`.

## ❓ Solución de Problemas

### "nvm: command not found"

El directorio de instalación no está en el PATH. Revisa la sección de [Configuración del PATH](#🔧-configuración-del-path).

### Error de permisos en Linux/macOS

```bash
# Dar permisos de ejecución
chmod +x ~/.local/bin/nvm

# O instalar en ubicación con permisos de usuario
nvm install-self --dir ~/.local/bin
```

### Error de permisos en Windows

Ejecuta PowerShell como Administrador o instala en un directorio donde tengas permisos de escritura:

```powershell
nvm install-self --dir "$env:LOCALAPPDATA\Programs\nvm"
```

### Checksum no coincide

Esto puede indicar una descarga corrupta o modificada. Intenta:

1. Descargar de nuevo
2. Verificar que estás descargando desde el repositorio oficial
3. Reportar el problema en GitHub Issues

### "Asset not found for your platform"

Tu combinación de sistema operativo y arquitectura puede no estar soportada aún. Puedes:

1. Compilar desde código fuente
2. Solicitar soporte para tu plataforma en GitHub Issues

## 🆘 Obtener Ayuda

- **Documentación:** [README.md](./README.md)
- **Issues:** [GitHub Issues](https://github.com/FreddyCamposeco/nvm-rs/issues)
- **Discusiones:** [GitHub Discussions](https://github.com/FreddyCamposeco/nvm-rs/discussions)

## 📚 Referencias

- [Repositorio GitHub](https://github.com/FreddyCamposeco/nvm-rs)
- [Releases](https://github.com/FreddyCamposeco/nvm-rs/releases)
- [Documentación Completa](./README.md)
