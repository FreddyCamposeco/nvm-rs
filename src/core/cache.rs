use crate::config::Config;
use crate::core::versions::NodeVersion;
use anyhow::Result;
use std::fs;
use std::time::{Duration, SystemTime};

/// Obtiene las versiones remotas, usando caché si está disponible y válido
pub async fn get_cached_versions(config: &Config) -> Result<Vec<NodeVersion>> {
    let cache_file = config.cache_file();

    // Verificar si el caché existe y no está expirado
    if cache_file.exists() {
        if let Ok(metadata) = fs::metadata(&cache_file) {
            if let Ok(modified) = metadata.modified() {
                let age = SystemTime::now().duration_since(modified)?;
                let max_age = Duration::from_secs(config.cache_duration_minutes * 60);

                if age < max_age {
                    // Leer del caché
                    let content = fs::read_to_string(&cache_file)?;
                    let versions: Vec<NodeVersion> = serde_json::from_str(&content)?;
                    return Ok(versions);
                }
            }
        }
    }

    // Si no hay caché válido, descargar y guardar
    let versions = fetch_remote_versions().await?;
    save_cache(config, &versions)?;
    Ok(versions)
}

/// Descarga las versiones desde nodejs.org
async fn fetch_remote_versions() -> Result<Vec<NodeVersion>> {
    let url = "https://nodejs.org/dist/index.json";
    let response = reqwest::get(url).await?;
    let versions: Vec<NodeVersion> = response.json().await?;
    Ok(versions)
}

/// Guarda las versiones en el caché
fn save_cache(config: &Config, versions: &[NodeVersion]) -> Result<()> {
    let cache_file = config.cache_file();

    // Crear directorio si no existe
    if let Some(parent) = cache_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(versions)?;
    fs::write(cache_file, content)?;
    Ok(())
}

/// Fuerza la actualización del caché
#[allow(dead_code)] // Will be used in Phase 2 (ls-remote update)
pub async fn update_cache(config: &Config) -> Result<()> {
    let versions = fetch_remote_versions().await?;
    save_cache(config, &versions)?;
    Ok(())
}

/// Obtiene las versiones instaladas localmente
pub fn get_installed_versions(config: &Config) -> Result<Vec<String>> {
    let versions_dir = config.versions_dir();

    if !versions_dir.exists() {
        return Ok(vec![]);
    }

    let mut versions = Vec::new();

    for entry in fs::read_dir(&versions_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    // Solo versiones que comienzan con 'v' y tienen formato semántico
                    if name_str.starts_with('v')
                        && name_str.len() > 1
                        && name_str.chars().nth(1).map(|c| c.is_ascii_digit()).unwrap_or(false)
                    {
                        versions.push(name_str.to_string());
                    }
                }
            }
        }
    }

    // Ordenar versiones (más reciente primero)
    versions.sort_by(|a, b| b.cmp(a));

    Ok(versions)
}

/// Obtiene información sobre el caché de versiones remotas
#[allow(dead_code)] // Will be used in Phase 3 (stats command)
pub fn get_cache_info(config: &Config) -> Result<CacheInfo> {
    let cache_file = config.cache_file();

    let mut info = CacheInfo {
        exists: cache_file.exists(),
        size_bytes: 0,
        last_updated: None,
        expires_at: None,
        is_valid: false,
    };    if cache_file.exists() {
        if let Ok(metadata) = fs::metadata(&cache_file) {
            info.size_bytes = metadata.len();

            if let Ok(modified) = metadata.modified() {
                if let Ok(age) = SystemTime::now().duration_since(modified) {
                    let max_age = Duration::from_secs(config.cache_duration_minutes * 60);
                    info.is_valid = age < max_age;
                    info.last_updated = Some(age.as_secs());

                    // Calcular fecha de expiración
                    if let Some(expiry_time) = modified.checked_add(max_age) {
                        info.expires_at = Some(expiry_time);
                    }
                }
            }
        }
    }

    Ok(info)
}

/// Obtiene el tamaño total de la carpeta de caché
#[allow(dead_code)] // Will be used in Phase 3 (stats command)
pub fn get_cache_total_size(config: &Config) -> Result<u64> {
    let cache_dir = config.cache_file().parent().map(|p| p.to_path_buf());

    if let Some(dir) = cache_dir {
        if dir.exists() {
            return Ok(calculate_dir_size(&dir)?);
        }
    }

    Ok(0)
}

fn calculate_dir_size(path: &std::path::Path) -> Result<u64> {
    let mut total = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            total += metadata.len();
        } else if metadata.is_dir() {
            total += calculate_dir_size(&entry.path())?;
        }
    }

    Ok(total)
}

/// Información del caché
#[derive(Debug, Clone)]
pub struct CacheInfo {
    pub exists: bool,
    pub size_bytes: u64,
    pub last_updated: Option<u64>, // En segundos
    pub expires_at: Option<SystemTime>,
    pub is_valid: bool,
}

impl CacheInfo {
    /// Formato legible del tamaño
    #[allow(dead_code)]
    pub fn size_human_readable(&self) -> String {
        let units = ["B", "KB", "MB", "GB"];
        let mut size = self.size_bytes as f64;
        let mut unit_idx = 0;

        while size >= 1024.0 && unit_idx < units.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }

        format!("{:.2} {}", size, units[unit_idx])
    }

    /// Tiempo desde última actualización en formato legible
    #[allow(dead_code)]
    pub fn last_updated_human_readable(&self) -> String {
        if let Some(secs) = self.last_updated {
            if secs < 60 {
                format!("{} seconds ago", secs)
            } else if secs < 3600 {
                format!("{} minutes ago", secs / 60)
            } else if secs < 86400 {
                format!("{} hours ago", secs / 3600)
            } else {
                format!("{} days ago", secs / 86400)
            }
        } else {
            "Never".to_string()
        }
    }
}

#[cfg(test)]
mod cache_tests {
    use super::*;

    #[test]
    fn test_cache_info_size_human() {
        let info = CacheInfo {
            exists: true,
            size_bytes: 1024 * 1024,  // 1 MB
            last_updated: Some(3600),  // 1 hour
            expires_at: None,
            is_valid: true,
        };

        assert_eq!(info.size_human_readable(), "1.00 MB");
    }

    #[test]
    fn test_cache_info_time_human() {
        let info = CacheInfo {
            exists: true,
            size_bytes: 0,
            last_updated: Some(1800),  // 30 minutes
            expires_at: None,
            is_valid: true,
        };

        assert!(info.last_updated_human_readable().contains("minutes"));
    }
}

pub fn save_installed_cache(config: &Config, versions: &[String]) -> Result<()> {
    let cache_file = config.installed_cache_file();
    let content = serde_json::to_string_pretty(versions)?;
    fs::write(cache_file, content)?;
    Ok(())
}

/// Lee el caché de versiones instaladas
#[allow(dead_code)] // Will be used in Phase 5 (list)
pub fn get_installed_cache(config: &Config) -> Result<Vec<String>> {
    let cache_file = config.installed_cache_file();

    if !cache_file.exists() {
        return get_installed_versions(config);
    }

    // Verificar edad del caché
    if let Ok(metadata) = fs::metadata(&cache_file) {
        if let Ok(modified) = metadata.modified() {
            let age = SystemTime::now().duration_since(modified)?;
            let max_age = Duration::from_secs(config.installed_cache_duration_minutes * 60);

            if age < max_age {
                let content = fs::read_to_string(&cache_file)?;
                let versions: Vec<String> = serde_json::from_str(&content)?;
                return Ok(versions);
            }
        }
    }

    // Caché expirado, regenerar
    let versions = get_installed_versions(config)?;
    save_installed_cache(config, &versions)?;
    Ok(versions)
}

/// Actualiza el cache de versiones instaladas después de instalar/desinstalar
pub fn refresh_installed_cache(config: &Config) -> Result<()> {
    let versions = get_installed_versions(config)?;
    save_installed_cache(config, &versions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_get_installed_versions_empty() {
        let temp = TempDir::new().unwrap();
        let config = Config {
            nvm_dir: temp.path().to_path_buf(),
            node_mirror: "".to_string(),
            arch: "x64".to_string(),
            cache_duration_minutes: 15,
            installed_cache_duration_minutes: 5,
        };

        let versions = get_installed_versions(&config).unwrap();
        assert_eq!(versions.len(), 0);
    }

    #[test]
    fn test_save_and_load_cache() {
        let temp = TempDir::new().unwrap();
        let config = Config {
            nvm_dir: temp.path().to_path_buf(),
            node_mirror: "".to_string(),
            arch: "x64".to_string(),
            cache_duration_minutes: 15,
            installed_cache_duration_minutes: 5,
        };

        let versions = vec!["v20.0.0".to_string(), "v18.0.0".to_string()];
        save_installed_cache(&config, &versions).unwrap();

        let loaded = get_installed_cache(&config).unwrap();
        assert_eq!(loaded, versions);
    }
}
