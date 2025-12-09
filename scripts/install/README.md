# Scripts de Instalación y Desinstalación

Scripts para instalar y desinstalar nvm-rs en el sistema.

## Archivos

### `install.ps1` (PowerShell - Windows - Recomendado)

Script de instalación principal para Windows. Descarga el binario más reciente de GitHub Releases e instala nvm-rs.

**Uso:**

```powershell
# Instalación simple (desde navegador o iwr)
iwr -useb https://github.com/FreddyCamposeco/nvm-rs/releases/download/latest/install.ps1 | iex

# Instalación local
.\install.ps1
.\install.ps1 -Version v0.5.0        # Versión específica
.\install.ps1 -WithSelfUpdate        # Con capacidad de auto-actualización
```

**Características:**
- Descarga automática de la versión más reciente
- Extrae a `$env:USERPROFILE\.nvm\`
- Configura PATH automáticamente
- Crea shims en PowerShell y CMD
- Soporte para auto-actualización

### `install.sh` (Bash/Shell - Linux/macOS)

Script de instalación para sistemas Unix/Linux/macOS.

**Uso:**

```bash
# Instalación simple
curl -fsSL https://github.com/FreddyCamposeco/nvm-rs/releases/download/latest/install.sh | bash

# Instalación local
./install.sh
./install.sh --version v0.5.0       # Versión específica
```

**Características:**
- Descarga automática desde GitHub
- Extrae a `~/.nvm/`
- Configura PATH en shell profile
- Compatible con bash, zsh, fish

### `uninstall.ps1` (PowerShell - Windows)

Script de desinstalación para Windows. Elimina completamente nvm-rs del sistema.

**Uso:**

```powershell
.\uninstall.ps1
.\uninstall.ps1 -RemoveData         # También elimina datos y caché
.\uninstall.ps1 -Full               # Eliminación completa
```

**Características:**
- Elimina binarios instalados
- Limpia variables de entorno
- Remueve PATH entries
- Opción de eliminar datos de usuario

### `uninstall.sh` (Bash/Shell - Linux/macOS)

Script de desinstalación para sistemas Unix.

**Uso:**

```bash
./uninstall.sh
./uninstall.sh --remove-data        # También elimina datos
./uninstall.sh --full               # Eliminación completa
```

**Características:**
- Elimina archivos instalados
- Limpia configuración de shell
- Remueve PATH entries

## Ubicaciones de Instalación

### Windows

```
User Profile: %USERPROFILE%\.nvm\
├── bin/              (ejecutables)
├── data/             (caché de versiones)
├── nvm.cmd           (shim para CMD)
└── nvm.ps1           (shim para PowerShell)
```

### Linux/macOS

```
Home: ~/.nvm/
├── bin/              (ejecutables)
├── data/             (caché de versiones)
└── nvm               (ejecutable principal)
```

## Variables de Entorno

### Windows

- `NVM_HOME`: `%USERPROFILE%\.nvm`
- `NVM_BIN`: `%USERPROFILE%\.nvm\bin`
- `NVM_NODE`: Directorio donde se descargan las versiones de Node

### Linux/macOS

- `NVM_HOME`: `~/.nvm`
- `NVM_BIN`: `~/.nvm/bin`
- `NVM_NODE`: Directorio donde se descargan las versiones de Node

## Ejemplos de Uso

**Instalación rápida en Windows (PowerShell):**

```powershell
iwr -useb https://github.com/FreddyCamposeco/nvm-rs/releases/download/latest/install.ps1 | iex
nvm --version
```

**Instalación rápida en Linux/macOS:**

```bash
curl -fsSL https://github.com/FreddyCamposeco/nvm-rs/releases/download/latest/install.sh | bash
source ~/.bashrc  # o ~/.zshrc
nvm --version
```

**Desinstalación completa en Windows:**

```powershell
.\uninstall.ps1 -Full
```

## Troubleshooting

**Error: "nvm command not found" después de instalar**
- Reiniciar PowerShell/CMD después de la instalación
- Verificar que `%USERPROFILE%\.nvm\bin` está en PATH

**Error: "Access Denied" en Windows**
- Ejecutar PowerShell como Administrador
- Puede requerir permisos elevados para modificar el registro

**Error: "Cannot download asset"**
- Verificar conexión a GitHub
- Comprobar que la versión existe: `git tag`
- Usar versión específica: `./install.ps1 -Version v0.5.0`

## Links Útiles

- **GitHub Releases**: https://github.com/FreddyCamposeco/nvm-rs/releases
- **Instrucciones detalladas**: Ver `/INSTALLATION.md`
