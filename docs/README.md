# Documentación de nvm-rs

Este directorio contiene toda la documentación del proyecto nvm-rs.

## Archivos

### `CHANGELOG.md`

Historial completo de cambios y versiones de nvm-rs. Documentación de todas las características agregadas, mejoras y correcciones por versión.

**Audiencia:** Desarrolladores y usuarios que deseen ver el historial de cambios.

### `RELEASE_NOTES_v0.5.0.md`

Notas de la versión actual (v0.5.0). Incluye características principales, cambios, mejoras y cualquier información importante para esta versión.

**Audiencia:** Usuarios finales interesados en la versión actual.

### `QUICK_RELEASE.md`

Guía rápida para realizar releases de nvm-rs. Contiene pasos simplificados para compilar, validar y publicar nuevas versiones en GitHub.

**Audiencia:** Desarrolladores que realizan releases.

### `MACOS_APPLE_SILICON.md`

Guía completa de instalación y configuración de nvm-rs en macOS Apple Silicon (ARM64). Incluye:
- Instalación desde binario precompilado
- Configuración automática con `nvm doctor --fix`
- Troubleshooting de problemas comunes
- Información sobre cómo construir desde código fuente

**Audiencia:** Usuarios de macOS con chips Apple Silicon (M1, M2, M3, etc.).

### `DOCTOR_COMMAND.md`

Documentación completa del comando `nvm doctor`. Describe:
- Cómo usar el comando para diagnosticar problemas
- Opción `--fix` para auto-configurar el entorno
- Explicación de cada verificación que realiza
- Soluciones para problemas típicos

**Audiencia:** Todos los usuarios que desean diagnosticar o auto-configurar su instalación.

## Documentación Adicional

- **BUILD_GUIDE.md**: Ver en `/scripts/BUILD_GUIDE.md` - Guía completa de compilación cross-platform
- **scripts/README.md**: Ver en `/scripts/README.md` - Documentación de scripts de build y deploy
- **README.md**: Ver en `/README.md` - Documentación general del proyecto
- **INSTALLATION.md**: Ver en `/INSTALLATION.md` - Guía de instalación
- **CONTRIBUTING.md**: Ver en `/CONTRIBUTING.md` - Guía para contribuidores

## Estructura del Proyecto

```
nvm-rs/
├── src/                  # Código fuente Rust
├── scripts/              # Scripts de build, install, deploy
├── locales/              # Archivos de traducción (i18n)
├── docs/                 # Documentación (este directorio)
├── release-builds/       # Binarios compilados
├── target/               # Artefactos de compilación (cargo)
├── Cargo.toml            # Configuración de cargo
├── LICENSE               # Licencia del proyecto
├── README.md             # Documentación principal
├── VERSION.md            # Información de versión
├── CONTRIBUTING.md       # Guía de contribuciones
└── INSTALLATION.md       # Guía de instalación
```

## Quick Links

- **GitHub**: <https://github.com/FreddyCamposeco/nvm-rs>
- **Releases**: <https://github.com/FreddyCamposeco/nvm-rs/releases>
- **Build Scripts**: `./scripts/`
- **Versión Actual**: v0.5.1 (ver VERSION.md)

## Plataformas Soportadas

nvm-rs v0.5.1 incluye soporte completo para:
- 🪟 **Windows**: x64, ARM64
- 🐧 **Linux**: x64, ARM64  
- 🍎 **macOS**: x64, ARM64 (Apple Silicon)
