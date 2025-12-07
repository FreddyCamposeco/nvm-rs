# Plan de Desarrollo - v0.4.0

## 🎯 Objetivos Principales

Implementar tres features críticas para mejorar la experiencia del usuario:

1. **Detección de Node.js del sistema** - Identificar instalaciones de Node.js existentes
2. **Cache de versiones remotas** - Caché persistente para mejorar performance
3. **Command `stats`** - Mostrar resumen de instalación

---

## 📋 Feature 1: Detección de Node.js del sistema

### Descripción

Comando para detectar e identificar instalaciones de Node.js en el sistema sin haber sido instaladas por nvm.

### Ubicación de Búsqueda

**Windows:**
- `C:\Program Files\nodejs\`
- `C:\Program Files (x86)\nodejs\`
- `%USERPROFILE%\AppData\Local\Programs\nodejs\`
- PATH (ejecutar `where node`)

**Linux/macOS:**
- `/usr/local/bin/node`
- `/usr/bin/node`
- `$HOME/.local/bin/node`
- PATH (ejecutar `which node`)

### Implementación

**Archivo**: `src/core/detection.rs` (nuevo)

```rust
pub struct SystemNodeInfo {
    pub path: PathBuf,
    pub version: String,
    pub npm_version: Option<String>,
}

pub fn detect_system_node() -> Option<SystemNodeInfo> {
    // Lógica de detección
}

pub fn find_all_node_installations() -> Vec<SystemNodeInfo> {
    // Encontrar múltiples instalaciones
}
```

**Integración en main.rs:**

```bash
nvm doctor --all              # Mostrar Node.js del sistema
nvm list-system              # Listar instalaciones del sistema
```

### Casos de Uso

1. Usuario con Node.js instalado globalmente quiere saber qué versión es
2. Migrar instalación existente a nvm
3. Diagnosticar conflictos entre múltiples instalaciones

---

## 📋 Feature 2: Cache de Versiones Remotas

### Descripción

Implementar caché persistente para la lista de versiones remotas (ls-remote) para mejorar performance.

### Especificación

**Ubicación del caché:**
- `~/.nvm/cache/versions.json` (persistente)
- `~/.nvm/cache/versions.meta.json` (metadata con timestamp)

**Duración del caché:**
- Por defecto: 24 horas
- Forzar refresh: `nvm ls-remote --no-cache`

**Contenido del caché:**

```json
{
  "versions": [
    {
      "version": "v20.10.0",
      "lts": "jod",
      "date": "2024-01-04",
      "files": ["node-v20.10.0-win-x64.zip", ...]
    }
  ],
  "cached_at": "2025-12-07T10:30:00Z",
  "expires_at": "2025-12-08T10:30:00Z"
}
```

### Implementación

**Archivo**: `src/core/cache.rs` (extender existente)

**Nuevas funciones:**

```rust
pub fn get_remote_versions(use_cache: bool) -> Result<Vec<Version>> {
    if use_cache && is_cache_valid() {
        return load_versions_from_cache();
    }
    // Descargar desde nodejs.org
    let versions = fetch_remote_versions()?;
    save_to_cache(&versions)?;
    Ok(versions)
}

fn is_cache_valid() -> bool {
    // Verificar que no haya expirado (24h)
}

fn load_versions_from_cache() -> Result<Vec<Version>> {
    // Leer de ~/.nvm/cache/versions.json
}

fn save_to_cache(versions: &[Version]) -> Result<()> {
    // Guardar a ~/.nvm/cache/versions.json
}
```

### Beneficios

- `nvm ls-remote` será ~50x más rápido en segundas llamadas
- Funciona offline (mientras caché sea válido)
- Reduce carga en nodejs.org

### Casos de Uso

1. Usuario ejecuta `nvm ls-remote` múltiples veces rápido
2. Conexión de internet lenta
3. Trabajo offline

---

## 📋 Feature 3: Command `stats`

### Descripción

Mostrar resumen estadístico de la instalación de nvm.

### Output Esperado

```
📊 NVM Statistics
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📦 Installation Info:
   Version: v0.4.0
   Location: C:\Users\Freddy\.nvm
   Binaries: 3.2 MB

🔄 Node.js Versions:
   Installed: 5 versions
   Active: v20.10.0 (node)
   Total size: 1.2 GB

   Versions:
   - v18.17.0 (lts/hydrogen) - 250 MB
   - ▶ v20.10.0 (lts/jod) - 280 MB
   - v22.11.0 (latest) - 290 MB
   - v19.9.0 - 270 MB
   - v21.0.0 - 270 MB

🏷️ Aliases: 3
   - default → v20.10.0
   - stable → v22.11.0
   - my-project → v18.17.0

💾 Cache:
   Cache dir: C:\Users\Freddy\.nvm\cache
   Size: 45 MB

🌐 Remote versions:
   Last update: 2025-12-07 10:30
   Cache age: 2 hours
   LTS versions available: 8
   Latest version: v22.11.0

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Implementación

**Archivo**: `src/commands/stats.rs` (nuevo)

```rust
pub struct Stats {
    pub nvm_version: String,
    pub nvm_location: PathBuf,
    pub nvm_size: u64,

    pub installed_versions: Vec<VersionStats>,
    pub active_version: String,
    pub total_node_size: u64,

    pub aliases: Vec<(String, String)>,
    pub cache_info: CacheInfo,
    pub remote_info: RemoteInfo,
}

pub fn get_stats() -> Result<Stats> {
    // Recopilar toda la información
}
```

**Integración:**

```bash
nvm stats              # Mostrar estadísticas
nvm stats --json      # Output JSON para scripting
```

### Casos de Uso

1. Usuario quiere ver cuánto espacio ocupa nvm
2. Auditar instalaciones para limpiar
3. Reportar estado del sistema

---

## 🔧 Plan de Implementación

### Fase 1: Detección de Node.js (2-3 días)
- [ ] Crear `src/core/detection.rs`
- [ ] Implementar búsqueda en rutas comunes
- [ ] Integrar en `nvm doctor`
- [ ] Tests de detección
- [ ] Commit: `feat(v0.4.0): detección de Node.js del sistema`

### Fase 2: Cache de Versiones Remotas (2-3 días)
- [ ] Extender `src/core/cache.rs`
- [ ] Implementar persistencia JSON
- [ ] Sistema de expiración (24h)
- [ ] Flag `--no-cache` para forzar refresh
- [ ] Tests de caché
- [ ] Commit: `feat(v0.4.0): cache de versiones remotas`

### Fase 3: Command `stats` (2-3 días)
- [ ] Crear `src/commands/stats.rs`
- [ ] Recopilar información del sistema
- [ ] Formatear output con colores
- [ ] Output JSON (`--json`)
- [ ] Tests
- [ ] Commit: `feat(v0.4.0): comando stats`

### Fase 4: Integración y Pruebas (1-2 días)
- [ ] Código limpio (clippy)
- [ ] Tests exhaustivos
- [ ] Actualizar README.md
- [ ] Actualizar CHANGELOG.md
- [ ] Commit final

### Fase 5: Merge y Release (1 día)
- [ ] Merge v0.4.0-dev → main
- [ ] Tag v0.4.0
- [ ] Publicar en GitHub Releases

---

## 📊 Estimación de Esfuerzo

| Feature | Líneas de Código | Tiempo | Complejidad |
|---------|-----------------|--------|-------------|
| Detección Node.js | 200-300 | 2-3 días | Media |
| Cache Versiones | 150-200 | 2-3 días | Media |
| Command Stats | 300-400 | 2-3 días | Media |
| Tests + integración | 200-300 | 1-2 días | Baja |
| **Total** | **850-1200** | **7-11 días** | - |

---

## 🧪 Testing Strategy

### Unit Tests

- Detección de Node.js en diferentes rutas
- Validación de caché (expiraciones, actualizaciones)
- Cálculo de estadísticas

### Integration Tests

- `nvm doctor --all` detecta Node.js del sistema
- `nvm ls-remote` usa caché en segunda llamada
- `nvm stats` muestra información correcta

### Manual Testing

- Verificar en Windows, Linux, macOS
- Probar con múltiples instalaciones de Node.js
- Validar output formato (colors, alineación)

---

## 📝 Cambios en Documentación

### README.md
- Agregar ejemplos de `nvm doctor --all`
- Documentar `nvm stats`
- Explicar caché (`--no-cache`)

### CHANGELOG.md
- Sección v0.4.0 con features nuevas

### Nuevos documentos
- `_docs/SYSTEM_DETECTION.md` - Guía de detección
- `_docs/CACHE_STRATEGY.md` - Estrategia de caché

---

## ✅ Checklist de Finalización

- [ ] Todas las features implementadas
- [ ] 0 warnings en clippy
- [ ] 100% tests pasando
- [ ] Documentación actualizada
- [ ] README.md actualizado
- [ ] v0.4.0-dev merged a main
- [ ] Tag v0.4.0 creado
- [ ] Binarios publicados en Releases

---

## 🚀 Próximas Fases

### v0.5.0 (después de v0.4.0)
- Configuración desde archivo `nvm.toml`
- LTS labels avanzados
- Mejoras en `update-self`

### v1.0.0
- Release Production
- Testing exhaustivo multiplataforma
- Documentación final
