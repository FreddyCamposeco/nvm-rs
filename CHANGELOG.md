# Changelog

Todos los cambios notables de este proyecto serán documentados en este archivo.

El formato está basado en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [0.1.0] - 2025-10-21

### 🎉 Release Inicial - Migración Completa

Primera versión funcional de nvm-rs, completando la migración de nvm-windows (PowerShell) a Rust con soporte multiplataforma.

### Added

#### Core Features
- ✨ **Gestión de versiones de Node.js**: Instalar, desinstalar, y cambiar entre versiones
- 🔍 **Búsqueda de versiones remotas**: Listar versiones disponibles desde nodejs.org
- 📦 **Sistema de instalación**: Descarga, verificación SHA256, y extracción automática
- 🔗 **Symlinks multiplataforma**: Junctions en Windows, symlinks en Unix/macOS
- 📄 **Soporte .nvmrc**: Detección automática de archivos .nvmrc
- 🎨 **Colores ANSI**: Interfaz colorida y amigable
- 🌍 **Internacionalización**: Soporte para Español e Inglés

#### Comandos Implementados (13)
- `nvm install <version>` - Instalar versión de Node.js
- `nvm uninstall <version> [--force]` - Desinstalar versión
- `nvm use <version>` - Cambiar a una versión específica
- `nvm ls` - Listar versiones instaladas con formato
- `nvm ls-remote [--lts]` - Listar versiones remotas disponibles
- `nvm current` - Mostrar versión actual activa
- `nvm alias <name> <version>` - Crear alias personalizado
- `nvm unalias <name>` - Eliminar alias
- `nvm aliases` - Listar todos los aliases
- `nvm cleanup [--yes]` - Limpiar versiones antiguas (mantiene LTS)
- `nvm doctor` - Diagnóstico del sistema
- `nvm self-update` - Auto-actualización (feature opcional)
- `nvm lang <locale>` - Cambiar idioma (es/en)

#### Sistema de Aliases
- 🏷️ Almacenamiento persistente en JSON
- ✅ Validación de nombres de alias
- 🔄 Resolución automática en comandos install/use
- 📝 Integración con aliases especiales (latest, lts, lts/*)

#### Sistema de Limpieza
- 🧹 Comando cleanup para eliminar versiones no usadas
- 🛡️ Protección automática de versión actual
- 🔰 Protección automática de versiones LTS
- ✅ Confirmación interactiva (skip con --yes)

#### Auto-Actualización
- 🔄 Integración con GitHub Releases
- ⚙️ Feature flag opcional (`--features self-update`)
- 📊 Detección de nueva versión disponible
- 📥 Descarga e instalación automática

### Technical Details

#### Architecture
- 🦀 **Rust 2021 Edition**
- 📦 **30+ Dependencies** cuidadosamente seleccionadas
- 🧪 **28 Tests Unitarios** con cobertura alta
- 📂 **Arquitectura Modular**: core/, utils/, i18n
- 🔧 **CLI con Clap v4**: Parsing de argumentos robusto

#### Performance
- ⚡ **Compilación**: 22s (release), 34s (release + self-update)
- 🚀 **Ejecución**: Más rápido que scripts PowerShell
- 💾 **Cache Inteligente**: Expiración automática de 15min
- 📊 **~3,500 líneas** de código Rust

#### Dependencies Principales
- `clap` - CLI parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` - Serialización
- `sha2` - Checksums
- `zip` / `tar` / `flate2` - Extracción de archivos
- `junction` (Windows) - Junctions
- `colored` - Colores ANSI
- `self_update` (opcional) - Auto-actualización

### Testing
- ✅ 28/28 tests unitarios pasando
- 🧪 Tests para aliases, cache, download, extract, symlink, versions
- 🔍 Cobertura de casos edge
- ⚠️ 0 warnings de compilación

### Plataformas Soportadas
- ✅ Windows (x64)
- ✅ Linux (x64)
- ✅ macOS (x64, ARM64)

### Idiomas
- 🇪🇸 Español
- 🇬🇧 English

### Known Limitations
- `set-default` comando no implementado (planeado para v0.2.0)
- Auto-update requiere compilar con feature flag
- Permisos de administrador pueden ser necesarios en Windows para symlinks

### Migration Notes
Este release completa la migración de nvm-windows (PowerShell) a Rust:
- ✅ Todas las funcionalidades principales migradas
- ✅ Comportamiento compatible con nvm-windows
- ✅ Mejor rendimiento y experiencia de usuario
- ✅ Soporte multiplataforma nativo

### Contributors
- Freddy Camposeco ([@FreddyCamposeco](https://github.com/FreddyCamposeco))

---

## [Unreleased]

### Planned for v0.2.0
- [ ] Comando `set-default`
- [ ] Integración automática con PATH
- [ ] CI/CD con GitHub Actions
- [ ] Releases automatizadas para múltiples plataformas
- [ ] Benchmarks de performance
- [ ] Documentación mejorada

---

[0.1.0]: https://github.com/FreddyCamposeco/nvm-rs/releases/tag/v0.1.0
[Unreleased]: https://github.com/FreddyCamposeco/nvm-rs/compare/v0.1.0...HEAD
