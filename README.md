# nvm-rs

Node Version Manager implementado en Rust - Multiplataforma (Windows, Linux, macOS)

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

## 🚀 Estado del Proyecto

**Versión**: 0.1.0  
**Estado**: ✅ **8/8 Fases Completadas** - Totalmente Funcional

### ✅ Todas las Fases Completadas (21 Oct 2025)

- ✅ **Fase 1**: Fundamentos Core (CLI, i18n, config, cache)
- ✅ **Fase 2**: Gestión de Versiones Remotas (ls-remote, HTTP, filtros)
- ✅ **Fase 3**: Instalación de Versiones (download, extract, checksums)
- ✅ **Fase 4**: Comando Use (symlinks, .nvmrc, switching)
- ✅ **Fase 5**: Listar Versiones Instaladas (ls, formateo, ordenamiento)
- ✅ **Fase 6**: Sistema de Aliases (alias, unalias, aliases)
- ✅ **Fase 7**: Cleanup & Maintenance (uninstall, cleanup)
- ✅ **Fase 8**: Self-Update (auto-actualización opcional)

**Tests**: 28/28 pasando ✓  
**Comandos Funcionales**: 13  
**Idiomas**: Español e Inglés

## ✨ Características

### Core Features

- 🔄 **Gestión de Versiones**: Instala, desinstala y cambia entre versiones de Node.js
- 🔍 **Búsqueda Inteligente**: Lista versiones remotas con filtros (LTS, platform)
- ✅ **Verificación de Integridad**: Checksums SHA256 automáticos
- 🔗 **Symlinks Multiplataforma**: Junctions en Windows, symlinks en Unix
- 📦 **Cache Inteligente**: Cache de versiones con expiración automática

### Advanced Features

- 🏷️ **Sistema de Aliases**: Crea aliases personalizados para versiones
- 🧹 **Limpieza Automática**: Elimina versiones antiguas manteniendo LTS
- 🔄 **Auto-Update**: Actualización automática desde GitHub Releases (opcional)
- 🌍 **Multiidioma**: Interfaz en Español e Inglés
- 📄 **.nvmrc Support**: Detección automática de archivos .nvmrc

## 📦 Instalación

### Desde Binarios Pre-compilados

```bash
# Descargar desde GitHub Releases
# https://github.com/FreddyCamposeco/nvm-rs/releases

# Windows
# Descargar nvm-windows-x64.exe

# Linux/macOS
# Descargar nvm-linux-x64 o nvm-macos-x64
chmod +x nvm-*
sudo mv nvm-* /usr/local/bin/nvm
```

### Compilar desde el Código Fuente

```bash
# Clonar el repositorio
git clone https://github.com/FreddyCamposeco/nvm-rs.git
cd nvm-rs

# Compilar (versión estándar)
cargo build --release

# Compilar con auto-update
cargo build --release --features self-update

# El binario estará en target/release/nvm
```

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

### Limpieza y Mantenimiento

```bash
# Limpiar versiones no usadas (mantiene actual y LTS)
nvm cleanup

# Limpiar sin confirmación
nvm cleanup --yes

# Diagnóstico del sistema
nvm doctor
```

### Auto-Actualización

```bash
# Actualizar nvm (solo si se compiló con --features self-update)
nvm self-update
```

### Configuración

```bash
# Cambiar idioma
nvm lang es    # Español
nvm lang en    # English
```

## 🌍 Internacionalización

Configure el idioma mediante:

```bash
# Variable de entorno
export NVM_LANG=es    # En Unix
$env:NVM_LANG="es"    # En Windows PowerShell

# O usando el comando
nvm lang es
```

**Idiomas soportados:**

- `en` - English (default)
- `es` - Español

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
```

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
|----------|-------------|---------|
| `NVM_DIR` | Directorio de instalación | `~/.nvm` |
| `NVM_LANG` | Idioma de la interfaz | `en` |
| `NODE_MIRROR` | Mirror de Node.js | `https://nodejs.org/dist` |
| `NO_COLOR` | Desactivar colores | - |

## 📊 Estadísticas del Proyecto

- **Líneas de Código**: ~3,500
- **Módulos**: 13
- **Tests Unitarios**: 28
- **Comandos**: 13
- **Dependencias**: ~30
- **Tiempo de Compilación**: 22s (release), 34s (release + self-update)

## 🎯 Comandos Disponibles

| Comando | Descripción |
|---------|-------------|
| `nvm install <version>` | Instalar versión de Node.js |
| `nvm uninstall <version>` | Desinstalar versión |
| `nvm use <version>` | Cambiar a una versión |
| `nvm ls` | Listar versiones instaladas |
| `nvm ls-remote [--lts]` | Listar versiones remotas |
| `nvm current` | Mostrar versión actual |
| `nvm alias <name> <ver>` | Crear alias |
| `nvm unalias <name>` | Eliminar alias |
| `nvm aliases` | Listar aliases |
| `nvm cleanup [--yes]` | Limpiar versiones antiguas |
| `nvm doctor` | Diagnóstico del sistema |
| `nvm self-update` | Actualizar nvm (opcional) |
| `nvm lang <locale>` | Cambiar idioma |

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

- Inspirado en [nvm-windows](https://github.com/coreybutler/nvm-windows)
- Comunidad de Rust
- Proyecto Node.js

## 📚 Referencias

- [Documentación del Proyecto](./docs/)
- [Plan de Migración](MIGRATION_PLAN.md)
- [Changelog](CHANGELOG.md)
- [Node.js Official Site](https://nodejs.org)
- [Rust Programming Language](https://www.rust-lang.org)

---

**¿Encontraste un bug?** [Reporta un issue](https://github.com/FreddyCamposeco/nvm-rs/issues)  
**¿Tienes una idea?** [Inicia una discusión](https://github.com/FreddyCamposeco/nvm-rs/discussions)
