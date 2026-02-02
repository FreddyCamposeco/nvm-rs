use crate::error::{message, Result};
use serde::{Deserialize, Serialize};

/// Representa una versión de Node.js del índice remoto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeVersion {
    pub version: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub npm: Option<String>,
    #[serde(default)]
    pub v8: Option<String>,
    #[serde(default)]
    pub uv: Option<String>,
    #[serde(default)]
    pub zlib: Option<String>,
    #[serde(default)]
    pub openssl: Option<String>,
    #[serde(default)]
    pub modules: Option<String>,
    pub lts: LtsInfo,
    #[serde(default)]
    pub security: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LtsInfo {
    Bool(bool),
    Name(String),
}

impl Default for LtsInfo {
    fn default() -> Self {
        LtsInfo::Bool(false)
    }
}

impl LtsInfo {
    pub fn is_lts(&self) -> bool {
        match self {
            LtsInfo::Bool(b) => *b,
            LtsInfo::Name(_) => true,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            LtsInfo::Name(n) => Some(n.as_str()),
            _ => None,
        }
    }
}

/// Resuelve un alias o versión a una versión completa
#[allow(dead_code)] // Will be used in Phase 2-3 (ls-remote, install)
pub fn resolve_version(version: &str, available_versions: &[NodeVersion]) -> Result<String> {
    // Primero, intentar resolver como alias personalizado
    if let Ok(aliases) = super::aliases::Aliases::load() {
        if let Some(resolved) = aliases.resolve(version) {
            // El alias puede apuntar a otro alias o versión, continuar resolución
            if resolved != version {
                // Evitar recursión infinita
                return resolve_version(&resolved, available_versions);
            }
        }
    }

    // Si ya es una versión completa, normalizarla
    if version.starts_with('v') && version.matches('.').count() == 2 {
        return Ok(version.to_string());
    }

    // Si es una versión sin 'v', agregarla
    if version.matches('.').count() == 2 && !version.starts_with('v') {
        return Ok(format!("v{}", version));
    }

    // Resolver alias especiales
    let version_lower = version.to_lowercase();
    match version_lower.as_str() {
        "latest" => {
            if let Some(latest) = available_versions.first() {
                return Ok(latest.version.clone());
            }
            return Err(message("No versions available"));
        }
        "lts" => {
            if let Some(lts_version) = available_versions.iter().find(|v| v.lts.is_lts()) {
                return Ok(lts_version.version.clone());
            }
            return Err(message("No LTS version found"));
        }
        alias if alias.starts_with("lts/") => {
            let lts_name = &alias[4..];
            if let Some(lts_version) = available_versions.iter().find(|v| {
                if let Some(name) = v.lts.name() {
                    name.to_lowercase() == lts_name
                } else {
                    false
                }
            }) {
                return Ok(lts_version.version.clone());
            }
            return Err(message(format!("LTS version '{}' not found", lts_name)));
        }
        _ => {
            // Buscar por nombre de LTS directamente
            if let Some(lts_version) = available_versions.iter().find(|v| {
                if let Some(name) = v.lts.name() {
                    name.to_lowercase() == version_lower
                } else {
                    false
                }
            }) {
                return Ok(lts_version.version.clone());
            }

            return Err(message(format!("Unknown version or alias: {}", version)));
        }
    }
}

/// Normaliza una versión asegurando que comience con 'v'
#[allow(dead_code)] // Will be used in Phase 3 (install)
pub fn normalize_version(version: &str) -> String {
    if version.starts_with('v') {
        version.to_string()
    } else {
        format!("v{}", version)
    }
}

/// Extrae el número de versión sin el prefijo 'v'
#[allow(dead_code)] // Will be used in Phase 5 (list)
pub fn version_number(version: &str) -> &str {
    version.strip_prefix('v').unwrap_or(version)
}

/// Filters versions based on criteria
#[derive(Default)]
pub struct VersionFilter {
    pub lts_only: bool,
    pub latest_only: bool,
    pub platform: Option<String>,
}

impl VersionFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lts_only(mut self) -> Self {
        self.lts_only = true;
        self
    }

    #[allow(dead_code)] // Will be used in future enhancements for --latest flag
    pub fn latest_only(mut self) -> Self {
        self.latest_only = true;
        self
    }

    #[allow(dead_code)] // Will be used in future enhancements
    pub fn platform(mut self, platform: String) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn apply(&self, versions: Vec<NodeVersion>) -> Vec<NodeVersion> {
        let mut filtered = versions;

        // Filter by LTS
        if self.lts_only {
            filtered.retain(|v| v.lts.is_lts());
        }

        // Filter by platform (check if files array contains the platform-specific archive)
        if let Some(platform) = &self.platform {
            let platform = platform.to_lowercase();

            let (platform_key, extension) = if platform.contains("win") {
                ("win", "zip")
            } else if platform.contains("darwin") || platform.contains("mac") {
                ("darwin", "tar.gz")
            } else {
                ("linux", "tar.gz")
            };

            filtered.retain(|v| {
                v.files.iter().any(|f| {
                    let f = f.to_lowercase();
                    f.contains(platform_key) && f.ends_with(extension)
                })
            });
        }

        // Take only latest if requested
        if self.latest_only && !filtered.is_empty() {
            filtered = vec![filtered[0].clone()];
        }

        filtered
    }
}

/// Format version for display with color coding
/// Reserved for future phases (enhanced UI formatting)
#[allow(dead_code)]
pub fn format_version_display(version: &NodeVersion) -> String {
    let version_str = &version.version;

    if let Some(lts_name) = version.lts.name() {
        format!("  {} (LTS: {})", version_str, lts_name)
    } else {
        format!("  {}", version_str)
    }
}

/// Lee la versión actual desde el symlink "current" o desde archivo .nvm-version
/// Intenta primero leer desde .nvm-version (más confiable en Windows), luego desde symlink
pub fn get_current_version(config: &crate::config::Config) -> Option<String> {
    let current_link = config.current_dir();

    if !current_link.exists() {
        return None;
    }

    // Primero, intentar leer desde .nvm-version (persistencia)
    let version_file = current_link.join(".nvm-version");
    if version_file.exists() {
        if let Ok(content) = std::fs::read_to_string(&version_file) {
            let version = content.trim().to_string();
            if !version.is_empty() {
                return Some(version);
            }
        }
    }

    // Fallback: Leer el target del symlink
    if let Ok(target) = std::fs::read_link(&current_link)
        .or_else(|_| current_link.canonicalize())
    {
        // Extraer el nombre de la versión del path
        if let Some(version_name) = target.file_name() {
            if let Some(version_str) = version_name.to_str() {
                return Some(version_str.to_string());
            }
        }
    }

    None
}

/// Formatea una versión instalada para mostrar en `nvm ls`
/// Marca la versión actual con ▶ y muestra información LTS si está disponible
/// Usa indicadores Unicode y colores diferenciados
pub fn format_installed_version(
    version: &str,
    is_current: bool,
    available_versions: &[NodeVersion],
) -> String {
    use colored::Colorize;

    // Buscar información LTS de esta versión
    let node_version = available_versions
        .iter()
        .find(|v| v.version == version);

    let lts_info = node_version.and_then(|v| v.lts.name());

    // Indicador visual: ▶ para actual, ✓ para instalada, espacio para normal
    let marker = if is_current {
        "▶".green().bold()
    } else {
        "✓".cyan()
    };

    // Colorear versión
    let version_colored = if is_current {
        version.green().bold()
    } else {
        version.cyan()
    };

    // Información adicional (LTS, fecha, etc.)
    let extra_info = if let Some(lts_name) = lts_info {
        format!("(LTS: {})", lts_name).yellow()
    } else if let Some(nv) = node_version {
        if nv.security {
            " [security]".red()
        } else {
            "".normal()
        }
    } else {
        "".normal()
    };

    if extra_info.to_string().is_empty() {
        format!("{} {}", marker, version_colored)
    } else {
        format!("{} {} {}", marker, version_colored, extra_info)
    }
}

/// Ordena versiones semánticamente (más reciente primero)
pub fn sort_versions(versions: &mut Vec<String>) {
    versions.sort_by(|a, b| {
        let parse_version = |v: &str| -> (u32, u32, u32) {
            let v_clean = v.strip_prefix('v').unwrap_or(v);
            let parts: Vec<u32> = v_clean
                .split('.')
                .filter_map(|s| s.parse().ok())
                .collect();

            (
                parts.get(0).copied().unwrap_or(0),
                parts.get(1).copied().unwrap_or(0),
                parts.get(2).copied().unwrap_or(0),
            )
        };

        let a_ver = parse_version(a);
        let b_ver = parse_version(b);

        // Orden descendente (más reciente primero)
        b_ver.cmp(&a_ver)
    });
}

/// Lee el archivo .nvmrc en el directorio especificado
pub fn read_nvmrc(dir: &std::path::Path) -> Option<String> {
    let nvmrc_path = dir.join(".nvmrc");

    if !nvmrc_path.exists() {
        return None;
    }

    if let Ok(content) = std::fs::read_to_string(&nvmrc_path) {
        // Tomar la primera línea y eliminar whitespace
        let version = content.lines().next()?.trim().to_string();
        if !version.is_empty() {
            return Some(version);
        }
    }

    None
}

/// Busca un archivo .nvmrc en el directorio actual y padres
pub fn find_nvmrc_in_tree(start_dir: Option<&std::path::Path>) -> Option<(std::path::PathBuf, String)> {
    let mut current_dir = start_dir
        .map(|p| p.to_path_buf())
        .or_else(|| std::env::current_dir().ok())?;

    loop {
        if let Some(version) = read_nvmrc(&current_dir) {
            return Some((current_dir.join(".nvmrc"), version));
        }

        // Subir al directorio padre
        if !current_dir.pop() {
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_versions() -> Vec<NodeVersion> {
        vec![
            NodeVersion {
                version: "v20.10.0".to_string(),
                lts: LtsInfo::Name("Iron".to_string()),
                files: vec![],
                date: "".to_string(),
                npm: None,
                v8: None,
                uv: None,
                zlib: None,
                openssl: None,
                modules: None,
                security: false,
            },
            NodeVersion {
                version: "v18.19.0".to_string(),
                lts: LtsInfo::Name("Hydrogen".to_string()),
                files: vec![],
                date: "".to_string(),
                npm: None,
                v8: None,
                uv: None,
                zlib: None,
                openssl: None,
                modules: None,
                security: false,
            },
            NodeVersion {
                version: "v19.0.0".to_string(),
                lts: LtsInfo::Bool(false),
                files: vec![],
                date: "".to_string(),
                npm: None,
                v8: None,
                uv: None,
                zlib: None,
                openssl: None,
                modules: None,
                security: false,
            },
        ]
    }

    #[test]
    fn test_resolve_latest() {
        let versions = create_test_versions();
        let result = resolve_version("latest", &versions).unwrap();
        assert_eq!(result, "v20.10.0");
    }

    #[test]
    fn test_resolve_lts() {
        let versions = create_test_versions();
        let result = resolve_version("lts", &versions).unwrap();
        assert_eq!(result, "v20.10.0");
    }

    #[test]
    fn test_resolve_lts_name() {
        let versions = create_test_versions();
        let result = resolve_version("lts/hydrogen", &versions).unwrap();
        assert_eq!(result, "v18.19.0");
    }

    #[test]
    fn test_normalize_version() {
        assert_eq!(normalize_version("18.0.0"), "v18.0.0");
        assert_eq!(normalize_version("v18.0.0"), "v18.0.0");
    }
}
