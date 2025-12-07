# 🤝 Guía de Contribución

¡Gracias por tu interés en contribuir a **nvm-rs**!

Este documento proporciona pautas para contribuir al proyecto.

## 📋 Código de Conducta

- Sé respetuoso y constructivo en tus interacciones
- Mantén un ambiente inclusivo y acogedor
- Acepta críticas constructivas de manera profesional

## 🚀 Cómo Contribuir

### Reportar Bugs

Si encontraste un bug, por favor:

1. Verifica que no exista un issue similar
2. Usa la plantilla de bug report
3. Incluye:
   - Descripción clara del problema
   - Pasos para reproducir
   - Comportamiento esperado vs actual
   - Sistema operativo y versión de nvm-rs
   - Output relevante o mensajes de error

### Sugerir Funcionalidades

Para proponer nuevas características:

1. Abre un issue con la etiqueta "enhancement"
2. Describe claramente:
   - El problema que resuelve
   - La solución propuesta
   - Alternativas consideradas
   - Impacto en usuarios existentes

### Pull Requests

#### Antes de Empezar

1. **Fork** el repositorio
2. **Clone** tu fork localmente
3. **Crea una rama** desde `dev`:

   ```bash
   git checkout -b feature/tu-feature
   ```

#### Durante el Desarrollo

1. **Sigue el estilo de código**:
   - Usa `rustfmt` para formatear código
   - Ejecuta `cargo clippy` para linting
   - Mantén líneas bajo 100 caracteres cuando sea posible

2. **Escribe tests**:
   - Agrega tests unitarios para nueva funcionalidad
   - Asegura que todos los tests pasen: `cargo test`
   - Mantén o mejora la cobertura de código

3. **Documenta tus cambios**:
   - Agrega comentarios para lógica compleja
   - Actualiza README.md si es necesario
   - Actualiza CHANGELOG.md con tus cambios

4. **Commits**:
   - Usa mensajes descriptivos
   - Sigue Conventional Commits:
     - `feat:` Nueva funcionalidad
     - `fix:` Corrección de bug
     - `docs:` Cambios en documentación
     - `test:` Agregar o modificar tests
     - `refactor:` Refactorización sin cambio funcional
     - `chore:` Cambios en build, deps, etc.

   Ejemplo:

   ```
   feat: add support for .node-version files

   - Parse .node-version files in current directory
   - Fallback to .nvmrc if .node-version not found
   - Add tests for version file detection
   ```

#### Antes de Enviar

```bash
# 1. Formatea el código
cargo fmt

# 2. Ejecuta el linter
cargo clippy -- -D warnings

# 3. Ejecuta todos los tests
cargo test

# 4. Compila en release
cargo build --release

# 5. Si agregaste features, prueba con ellos
cargo build --release --features self-update
```

#### Enviar el PR

1. Push a tu fork:

   ```bash
   git push origin feature/tu-feature
   ```

2. Abre un Pull Request hacia la rama `dev`

3. Completa la plantilla del PR:
   - Descripción clara de cambios
   - Issue relacionado (si aplica)
   - Screenshots (si hay cambios visuales)
   - Checklist de verificación

4. Espera review y responde a comentarios

## 📁 Estructura del Proyecto

```
nvm-rs/
├── src/
│   ├── main.rs           # Entry point y CLI
│   ├── core/             # Lógica core
│   │   ├── mod.rs
│   │   ├── aliases.rs    # Sistema de aliases
│   │   ├── cache.rs      # Cache HTTP
│   │   ├── config.rs     # Configuración
│   │   ├── doctor.rs     # Diagnóstico
│   │   ├── install.rs    # Instalación de versiones
│   │   └── versions.rs   # Gestión de versiones
│   ├── utils/            # Utilidades
│   │   ├── mod.rs
│   │   ├── ansi.rs       # Colores ANSI
│   │   ├── http.rs       # Cliente HTTP
│   │   └── symlink.rs    # Symlinks multiplataforma
│   └── i18n/             # Internacionalización
│       └── mod.rs
├── locales/              # Traducciones
│   ├── en.yaml
│   └── es.yaml
├── tests/                # Tests de integración
├── Cargo.toml            # Dependencias
└── README.md
```

## 🧪 Tests

### Ejecutar Tests

```bash
# Todos los tests
cargo test

# Tests específicos
cargo test test_parse_version

# Tests con output
cargo test -- --nocapture

# Tests de un módulo
cargo test core::aliases
```

### Escribir Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mi_funcionalidad() {
        let resultado = mi_funcion();
        assert_eq!(resultado, valor_esperado);
    }
}
```

## 🌍 Internacionalización

Para agregar un nuevo idioma:

1. Crea `locales/xx.yaml` (donde xx es el código ISO)
2. Copia todas las claves de `en.yaml`
3. Traduce los valores
4. Actualiza `src/i18n/mod.rs` para incluir el idioma
5. Agrega tests para el nuevo idioma

## 🎨 Estilo de Código

### Rust Style Guide

- Usa `snake_case` para funciones y variables
- Usa `PascalCase` para tipos y structs
- Usa `SCREAMING_SNAKE_CASE` para constantes
- Prefiere `&str` sobre `String` cuando sea posible
- Usa `Result<T, E>` para operaciones que pueden fallar
- Documenta funciones públicas con `///`

### Ejemplo

```rust
/// Parse a semantic version string.
///
/// # Arguments
/// * `version` - Version string (e.g., "20.11.0")
///
/// # Returns
/// * `Some((major, minor, patch))` if valid
/// * `None` if invalid
pub fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    // Implementación
}
```

## 📝 Documentación

- Documenta funciones públicas
- Incluye ejemplos en docstrings cuando sea útil
- Actualiza README.md para cambios user-facing
- Mantén CHANGELOG.md actualizado

## 🔄 Proceso de Review

1. **Automático**: CI ejecutará tests y linters
2. **Manual**: Maintainer revisará el código
3. **Feedback**: Responde a comentarios y realiza cambios
4. **Merge**: Una vez aprobado, se mergeará a `dev`

## 📮 Preguntas

Si tienes preguntas:

1. Revisa la documentación existente
2. Busca en issues cerrados
3. Abre un issue con la etiqueta "question"
4. Participa en Discussions (si están habilitadas)

## 🙏 Agradecimientos

¡Gracias por contribuir a nvm-rs! Cada contribución, grande o pequeña, es valiosa.

---

**Happy Coding!** 🦀
