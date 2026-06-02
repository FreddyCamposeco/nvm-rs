# nvm-rs

🚀 **Node Version Manager** implementado en Rust - Rápido, seguro y multiplataforma (v0.7.0)

[![Version](https://img.shields.io/github/v/release/FreddyCamposeco/nvm-rs?label=version)](https://github.com/FreddyCamposeco/nvm-rs/releases/latest) [![Rust](https://img.shields.io/badge/rust-1.91%2B-orange.svg)](https://www.rust-lang.org) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) [![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)](https://github.com/FreddyCamposeco/nvm-rs/releases) [![Downloads](https://img.shields.io/github/downloads/FreddyCamposeco/nvm-rs/total)](https://github.com/FreddyCamposeco/nvm-rs/releases)

## 📋 Tabla de Contenidos

- [Características](#-características)
- [Instalación Rápida](#-instalación-rápida)
- [Uso](#-uso)
- [Estructura de Directorios](#-estructura-de-directorios)
- [Comandos Disponibles](#-comandos-disponibles)
- [Configuración](#-configuración)
- [Desarrollo](#️-desarrollo)
- [Contribuir](#-contribuir)
- [Licencia](#-licencia)

## 🚀 Estado del Proyecto

**Versión**: 0.7.0
**Estado**: ✅ Producción - Completamente Funcional
**Plataformas Soportadas**: 
- 🪟 Windows (x64, ARM64)
- 🐧 Linux (x64, ARM64) 
- 🍎 macOS (x64, Apple Silicon ARM64)
- ✅ Todas con features idénticas y cross-platform homologado

## ✨ Características

- 🚀 **Rápido**: Escrito en Rust para máximo rendimiento
- 🔄 **Gestión Completa**: Instala, desinstala y cambia entre versiones de Node.js
- 🔍 **Búsqueda Inteligente**: Filtra versiones remotas por LTS, versión, código
- ✅ **Seguro**: Verificación de integridad con checksums SHA256
- 🔗 **Symlinks Inteligentes**: Junctions en Windows, symlinks en Unix (sin permisos admin)
- 📦 **Cache Eficiente**: Sistema de caché con expiración de 24 horas
- 🔎 **Detección de Sistema**: Identifica instalaciones de Node.js en el sistema
- 📊 **Estadísticas**: Comando `stats` para ver información de instalación
- 🏷️ **Sistema de Aliases**: Crea alias personalizados (default, stable, etc.)
- 🧹 **Auto-Limpieza**: Elimina versiones antiguas manteniendo LTS y versión activa
- 🔄 **Auto-Instalación**: Instala, actualiza y desinstala nvm desde GitHub Releases
- 🌍 **Multiidioma**: Interfaz completa en Español e Inglés
- 📄 **`.nvmrc` Support**: Detección automática en árbol de directorios
- 🐚 **Shell Integration**: Hook automático de `.nvmrc` al cambiar directorios (`shell-init`)
- 📦 **Migración de Paquetes**: Reinstala paquetes npm globales entre versiones (`reinstall-packages`)
- 🎯 **PATH Homologado**: Estructura consistente entre Windows, Linux y macOS

## 📦 Instalación Rápida

### Script de Instalación Automática (Recomendado)

**Windows (PowerShell)**

```powershell
# Instalación con un solo comando
iwr -useb https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.ps1 | iex

# O con opciones personalizadas
$env:NVM_VERSION="v0.7.0"; $env:NVM_INSTALL_DIR="C:\nvm"; iwr -useb https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.ps1 | iex
```

**Linux / macOS (Bash)**

```bash
# Instalación con un solo comando
curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.sh | bash

# O con opciones personalizadas
export NVM_VERSION="v0.7.0"
export NVM_INSTALL_DIR="$HOME/.nvm"
curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.sh | bash
```

**El script automáticamente:**

- ✅ Detecta tu sistema operativo y arquitectura
- ✅ Descarga la versión correcta desde GitHub Releases
- ✅ Verifica la integridad con checksums SHA256
- ✅ Instala el binario de nvm en `$NVM_HOME/bin` (homologado en todas plataformas)
- ✅ Configura variables de entorno (`NVM_HOME`, `NVM_BIN`, `NVM_NODE`)
- ✅ Crea automáticamente estructura de directorios (`versions/`, `current/bin/`, `cache/`, `alias/`)
- ✅ Agrega `$NVM_HOME/bin` y `$NVM_HOME/current/bin` al PATH
- ✅ Crea backup de versiones anteriores

### Gestión de nvm

```bash
# Instalar/actualizar usando nvm (si ya lo tienes instalado)
nvm install-self                 # Instalar última versión
nvm install-self -v v0.7.0      # Instalar versión específica
nvm update-self                  # Actualizar a la última versión
nvm uninstall-self              # Desinstalar nvm del sistema
```

### Instalación Manual

Para instalación manual o compilar desde código fuente, consulta la [Guía de Instalación Completa](INSTALLATION.md).

## 🔧 Uso

### Gestión de Versiones

```bash
# Listar versiones remotas disponibles
nvm ls-remote

# Listar solo versiones LTS
nvm ls-remote --lts

# Instalar una versión específica
nvm install 20.10.0
nvm install v22.21.0

# Instalar usando aliases
nvm install latest       # Última versión
nvm install lts          # Última LTS
nvm install lts/iron     # Última Iron LTS

# Listar versiones instaladas
nvm ls

# Cambiar a una versión
nvm use 20.10.0
nvm use lts

# Cambiar usando .nvmrc (si existe en el directorio actual)
nvm use

# Ver versión actual
nvm current

# Desinstalar una versión
nvm uninstall 20.10.0

# No se puede desinstalar la versión activa (usar --force)
nvm uninstall 22.21.0 --force
```

### Sistema de Aliases

```bash
# Crear un alias
nvm alias default 20.10.0
nvm alias stable lts
nvm alias my-project 22.21.0

# Listar todos los aliases
nvm aliases

# Usar un alias
nvm use default
nvm install stable

# Eliminar un alias
nvm unalias my-project
```

### Integración con Shell

```bash
# Auto-detectar .nvmrc al hacer cd (añadir al perfil)
eval "$(nvm shell-init bash)"       # ~/.bashrc
eval "$(nvm shell-init zsh)"        # ~/.zshrc
nvm shell-init fish | source        # ~/.config/fish/config.fish

# PowerShell - añadir a $PROFILE
Invoke-Expression (nvm shell-init powershell)
```

```bash
# Reinstalar paquetes globales de una versión anterior
nvm reinstall-packages v20.19.2    # Copia paquetes globales de v20 a la versión actual
```

### Limpieza y Mantenimiento

```bash
# Limpiar versiones no usadas (mantiene actual y LTS)
nvm cleanup

# Limpiar sin confirmación
nvm cleanup --yes

# Diagnóstico del sistema
nvm doctor
```

### Configuración y Utilidades

```bash
# Ver estadísticas de instalación
nvm stats                  # Formato tabular
nvm stats --json           # Formato JSON

# Diagnóstico del sistema (auto-fix opcional)
nvm doctor                 # Verifica instalación y configuración
nvm doctor --fix           # Auto-configura PATH y variables en Unix
nvm doctor --system        # Detecta Node.js del sistema
nvm doctor --all           # Muestra todas las instalaciones encontradas

# Cambiar idioma
nvm lang es    # Español
nvm lang en    # English

# Gestión de nvm mismo
nvm install-self           # Instalar nvm desde GitHub
nvm update-self            # Actualizar a la última versión
nvm uninstall-self         # Desinstalar nvm completamente
```

## 📁 Estructura de Directorios

nvm-rs utiliza una estructura consistente entre plataformas:

### Windows

```
%USERPROFILE%\.nvm\             # NVM_HOME
├── .version_cache.json
├── alias\                      # Aliases personalizados
├── bin\                        # $NVM_BIN (binario de nvm)
│   └── nvm.exe
├── cache\                      # Archivos descargados
├── current\                    # Junction a versión activa
│   ├── bin\                    # Junction → ..\versions\v{version}\bin ($NVM_NODE)
│   │   ├── node.exe
│   │   ├── npm.cmd
│   │   └── npx.cmd
│   └── .nvm-version            # Archivo con versión persistida
└── versions\                   # Versiones instaladas
    └── v18.17.0\
        ├── bin\
        │   ├── node.exe
        │   ├── npm.cmd
        │   └── npx.cmd
        └── lib\
```

### Linux / macOS

```
~/.nvm/                        # NVM_HOME
├── .version_cache.json
├── alias/                      # Aliases personalizados
├── bin/                        # $NVM_BIN (binario de nvm)
│   └── nvm
├── cache/                      # Archivos descargados
├── current/                    # Symlink a versión activa
│   ├── bin/                    # Symlink → ../versions/v{version}/bin ($NVM_NODE)
│   │   ├── node
│   │   ├── npm
│   │   └── npx
│   └── .nvm-version            # Archivo con versión persistida
└── versions/                   # Versiones instaladas
    └── v18.17.0/
        ├── bin/
        │   ├── node
        │   ├── npm
        │   └── npx
        └── lib/      # Cache de versiones remotas
```

**Variables de entorno configuradas:**

- `NVM_HOME`: Directorio base (homologado: `%USERPROFILE%\.nvm` en Windows, `~/.nvm` en Linux/macOS)
- `NVM_BIN`: Binario de nvm (homologado: `$NVM_HOME/bin` en todas plataformas)
- `NVM_NODE`: Node.js activo (homologado: `$NVM_HOME/current/bin` en todas plataformas)
- `PATH`: Incluye `$NVM_BIN` (nvm) y `$NVM_NODE` (Node.js activo)

Ver [PATH_STRUCTURE.md](PATH_STRUCTURE.md) para detalles completos sobre la estructura homologada.

## 🌍 Internacionalización

**Idiomas soportados:**

- 🇬🇧 `en` - English (default)
- 🇪🇸 `es` - Español

**Configurar idioma:**

```bash
# Variable de entorno
export NVM_LANG=es              # Unix
$env:NVM_LANG="es"              # Windows

# O usando el comando
nvm lang es
```

## 🏗️ Arquitectura

```
src/
├── main.rs              # Entry point y CLI
├── config.rs            # Configuración global
├── i18n.rs              # Sistema i18n
├── core/                # Lógica de negocio
│   ├── versions.rs      # Resolución de versiones
│   ├── cache.rs         # Sistema de caché
│   ├── download.rs      # Descarga de archivos
│   ├── extract.rs       # Extracción de archivos
│   ├── symlink.rs       # Gestión de symlinks
│   └── aliases.rs       # Sistema de aliases
└── utils/               # Utilidades
    ├── colors.rs        # Colores ANSI
    ├── http.rs          # Cliente HTTP
    └── mod.rs           # Helpers generales

locales/                 # Traducciones
├── en.yaml             # English
└── es.yaml             # Español
```

## 🛠️ Desarrollo

### Compilar y Ejecutar

```bash
# Compilar en modo debug
cargo build

# Ejecutar
cargo run -- ls-remote

# Compilar en modo release
cargo build --release

# Con feature self-update
cargo build --release --features self-update

# Compilar para plataformas específicas (requiere targets instalados)
cargo build --target aarch64-apple-darwin --release  # macOS ARM64
cargo build --target x86_64-apple-darwin --release   # macOS x64
cargo build --target x86_64-unknown-linux-gnu --release
cargo build --target aarch64-unknown-linux-gnu --release
cargo build --target x86_64-pc-windows-msvc --release
```

### Build Scripts

nvm-rs incluye scripts automatizados para compilar todas las plataformas:

```bash
# Usar build.sh en Linux/macOS o build-releases.ps1 en Windows PowerShell
./scripts/build/build.sh                    # Auto-detecta SO y compila
./scripts/build/build.sh --target macos-arm64  # Compilar específico
./scripts/build/build.sh --with-self-update    # Incluir capacidad self-update
```

Ver [BUILD_GUIDE.md](scripts/BUILD_GUIDE.md) para más detalles.

### Tests

```bash
# Ejecutar todos los tests
cargo test

# Tests con feature self-update
cargo test --features self-update

# Tests específicos
cargo test --bin nvm

# Con output verbose
cargo test -- --nocapture
```

### Documentación

```bash
# Generar y abrir documentación
cargo doc --open
```

## 📝 Variables de Entorno

| Variable | Descripción | Default |
|----------|-------------|----------|
| `NVM_HOME` | Directorio base de nvm | `~/.nvm` |
| `NVM_BIN` | Directorio del binario nvm | `$NVM_HOME/bin` |
| `NVM_NODE` | Directorio de Node.js activo | `$NVM_HOME/current/bin` |
| `NVM_LANG` | Idioma de la interfaz | `en` |
| `NODE_MIRROR` | Mirror de Node.js para descargas | `https://nodejs.org/dist` |
| `NO_COLOR` | Desactivar colores en la salida | - |

**Ejemplos de configuración:**

```bash
# Linux/macOS - Agregar a ~/.bashrc o ~/.zshrc
export NVM_HOME="$HOME/.nvm"
export NVM_BIN="$NVM_HOME/bin"
export NVM_NODE="$NVM_HOME/current/bin"
export NODE_MIRROR="https://mirrors.aliyun.com/nodejs-release"  # Mirror alternativo
export PATH="$NVM_BIN:$NVM_NODE:$PATH"
```

```powershell
# Windows PowerShell - Ejecutar una sola vez
[Environment]::SetEnvironmentVariable('NVM_HOME', "$env:USERPROFILE\.nvm", 'User')
[Environment]::SetEnvironmentVariable('NVM_BIN', "$env:NVM_HOME\bin", 'User')
[Environment]::SetEnvironmentVariable('NVM_NODE', "$env:NVM_HOME\current\bin", 'User')
[Environment]::SetEnvironmentVariable('NODE_MIRROR', 'https://nodejs.org/dist', 'User')
```

## 📊 Estadísticas del Proyecto

| Métrica | Valor |
|---------|-------|
| **Líneas de Código** | ~3,920 |
| **Módulos** | 17 |
| **Comandos** | 16 |
| **Idiomas** | 2 (Español, Inglés) |
| **Plataformas** | 6 (Windows x64/x86, Linux x64/ARM64, macOS x64/ARM64) |
| **Dependencias** | ~15 principales |

## 🎯 Comandos Disponibles

### Gestión de Node.js

| Comando | Descripción |
|---------|-------------|
| `nvm install <version>` | Instalar versión de Node.js (soporta aliases: latest, lts, lts/iron) |
| `nvm uninstall <version>` | Desinstalar versión (usa --force para desinstalar versión activa) |
| `nvm use [version]` | Cambiar a una versión (busca .nvmrc si no se especifica) |
| `nvm ls` | Listar versiones instaladas localmente |
| `nvm ls-remote [--lts]` | Listar versiones disponibles en nodejs.org |
| `nvm current` | Mostrar versión actualmente en uso |

### Sistema de Aliases

| Comando | Descripción |
|---------|-------------|
| `nvm alias <name> <ver>` | Crear alias personalizado (ej: default, stable) |
| `nvm unalias <name>` | Eliminar alias |
| `nvm aliases` | Listar todos los aliases configurados |

### Mantenimiento

| Comando | Descripción |
|---------|-------------|
| `nvm cleanup [--yes]` | Limpiar versiones no usadas (mantiene LTS y actual) |
| `nvm doctor [--all] [--system] [--fix]` | Diagnóstico del sistema (--fix: auto-configura PATH en Unix) |
| `nvm stats [--json]` | Mostrar estadísticas de instalación (formato texto o JSON) |

### Gestión de nvm

| Comando | Descripción |
|---------|-------------|
| `nvm install-self [-v ver] [-d dir]` | Instalar/reinstalar nvm desde GitHub Releases |
| `nvm update-self [-v ver]` | Actualizar nvm a la última versión disponible |
| `nvm uninstall-self [--yes]` | Desinstalar nvm del sistema |

### Configuración

| Comando | Descripción |
|---------|-------------|
| `nvm lang <locale>` | Cambiar idioma (es/en) |
| `nvm shell-init <shell>` | Generar script de integración para bash/zsh/fish/powershell |
| `nvm reinstall-packages <ver>` | Reinstalar paquetes npm globales de otra versión en la actual |

## 🤝 Contribuir

Las contribuciones son bienvenidas! Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

### Guías de Estilo

- Sigue las convenciones de Rust (usa `cargo fmt`)
- Agrega tests para nuevas funcionalidades
- Actualiza la documentación si es necesario
- Mantén los mensajes de commit descriptivos

## 📄 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles.

## 👤 Autor

**Freddy Camposeco**

- GitHub: [@FreddyCamposeco](https://github.com/FreddyCamposeco)
- Proyecto: [nvm-rs](https://github.com/FreddyCamposeco/nvm-rs)

## 🙏 Agradecimientos

- **nvm-rs reemplaza a nvm-windows** - Implementación mejorada multiplataforma en Rust (Windows, Linux, macOS)
- Comunidad de Rust
- Proyecto Node.js

## 📚 Documentación Adicional

- 📖 [Guía Completa de Instalación](INSTALLATION.md) - Métodos de instalación detallados
- 🗺️ [Estructura de Directorios y PATH](PATH_STRUCTURE.md) - Arquitectura homologada multiplataforma
- 📝 [Changelog](CHANGELOG.md) - Historial de cambios y versiones
- 🤝 [Guía de Contribución](CONTRIBUTING.md) - Cómo contribuir al proyecto
- 🔄 [Plan de Migración](MIGRATION_PLAN.md) - Roadmap y fases completadas

## 🔗 Enlaces Útiles

- [Node.js Official Site](https://nodejs.org)
- [Rust Programming Language](https://www.rust-lang.org)
- [GitHub Releases - nvm-rs](https://github.com/FreddyCamposeco/nvm-rs/releases)

<div align="center">

**¿Encontraste un bug?** → [Reporta un issue](https://github.com/FreddyCamposeco/nvm-rs/issues)
**¿Tienes una idea?** → [Inicia una discusión](https://github.com/FreddyCamposeco/nvm-rs/discussions)
**¿Te gusta el proyecto?** → [Dale una ⭐](https://github.com/FreddyCamposeco/nvm-rs)

Hecho con ❤️ y 🦀 por [Freddy Camposeco](https://github.com/FreddyCamposeco)

</div>
