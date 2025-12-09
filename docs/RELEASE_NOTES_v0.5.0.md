# Release Notes - v0.5.0

**Fecha de Lanzamiento**: Diciembre 7, 2025
**Versión Anterior**: v0.4.0

## 🎯 Resumen

Esta versión integra las capacidades de detección de Node.js del sistema desarrolladas en v0.4.0 directamente en el comando `doctor`, además de incluir mejoras de código y documentación completa actualizada.

## ✨ Nuevas Características

### 1. Integración de System Node Detection en `doctor`

El comando `doctor` ahora incluye flags para detectar instalaciones de Node.js:

```
# Detectar todas las instalaciones de Node.js
nvm doctor --all

# Solo mostrar Node.js del sistema (no gestionado por nvm)
nvm doctor --system

# Información general (comportamiento por defecto)
nvm doctor
```

**Funcionalidad incluida**:

- ✅ Detección automática de Node.js en PATH
- ✅ Búsqueda en ubicaciones del sistema (Program Files, /usr/local, ~/.local)
- ✅ Identificación de instalaciones gestionadas por nvm vs sistema
- ✅ Información completa: versión de Node.js y npm
- ✅ Soporte multiplataforma (Windows, Linux, macOS)

### 2. Documentación Actualizada

- ✅ README.md actualizado a v0.5.0 con todas las características
- ✅ CHANGELOG.md completo con v0.4.0 y v0.5.0
- ✅ VERSION.md con historial y matriz de plataformas
- ✅ Estadísticas del proyecto actualizadas

## 🐛 Mejoras Técnicas

### Limpieza de Código

- ✅ Eliminados 8 warnings de código muerto (`dead_code`)
- ✅ Agregados `#[allow(dead_code)]` para funciones reservadas para uso futuro
- ✅ **Compilación final: 0 errores, 1 warning residual (aceptable)**
- ✅ Código más limpio y mantenible

### Archivos modificados:

- `src/main.rs` - Integración de flags --all y --system en doctor
- `src/core/detection.rs` - Atributos `#[allow(dead_code)]`
- `src/core/cache.rs` - Atributos `#[allow(dead_code)]`
- `src/core/installer.rs` - Atributos `#[allow(dead_code)]`

## 📊 Estadísticas

- **Líneas añadidas**: +48
- **Archivos modificados**: 4
- **Commits**: 2
- **Warnings eliminados**: 8 → 1 (7 resueltos)

## 🚀 Instalación

### Windows (PowerShell)

```
$env:NVM_VERSION='v0.5.0'; iwr -useb https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install.ps1 | iex
```

### Linux / macOS (Bash)

```
export NVM_VERSION='v0.5.0'
curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install.sh | bash
```

### Actualización desde versión anterior

```
nvm update-self -v v0.5.0
```

## 📝 Notas de Actualización

### Desde v0.4.0

- Sin cambios breaking
- Compatible con configuraciones existentes
- Nuevos flags opcionales en `doctor`

### Desde v0.3.0 o anterior

- Revisa las notas de v0.4.0 para características intermedias
- El comando `stats` está disponible desde v0.4.0
- Cache extendido a 24 horas desde v0.4.0

## 🔄 Cambios Breaking

**Ninguno** - Esta versión es totalmente compatible con v0.4.0

## 🐛 Problemas Conocidos

Ninguno reportado.

## 🙏 Agradecimientos

Gracias a todos los que han contribuido reportando issues y sugerencias.

**Descarga**: [GitHub Releases](https://github.com/FreddyCamposeco/nvm-rs/releases/tag/v0.5.0)
**Documentación**: [README.md](https://github.com/FreddyCamposeco/nvm-rs/blob/main/README.md)
**Changelog Completo**: [CHANGELOG.md](https://github.com/FreddyCamposeco/nvm-rs/blob/main/CHANGELOG.md)
