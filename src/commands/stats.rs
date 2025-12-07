// Stats command - Show nvm installation summary
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::core::cache::get_cache_info;
use crate::core::aliases::Aliases;
use crate::core::cache::get_cache_total_size;

/// Estadísticas de instalación de nvm
#[derive(Debug, Clone)]
pub struct Stats {
    pub nvm_version: String,
    pub nvm_location: PathBuf,
    pub nvm_size: u64,

    pub installed_versions_count: usize,
    pub active_version: Option<String>,
    pub total_node_size: u64,

    pub aliases_count: usize,
    pub cache_size: u64,
    pub cache_valid: bool,
    pub cache_age: Option<u64>,
}

/// Obtener estadísticas del sistema
pub async fn get_stats(config: &Config) -> anyhow::Result<Stats> {
    let nvm_dir = config.nvm_dir.clone();

    // Tamaño de nvm binario
    let nvm_bin = nvm_dir.join("bin").join(
        if cfg!(target_os = "windows") {
            "nvm.exe"
        } else {
            "nvm"
        }
    );
    let nvm_size = if nvm_bin.exists() {
        fs::metadata(&nvm_bin)?.len()
    } else {
        0
    };

    // Versiones instaladas
    let versions_dir = nvm_dir.join("versions");
    let mut installed_count = 0;
    let mut total_size = 0;

    if versions_dir.exists() {
        for entry in fs::read_dir(&versions_dir)? {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    installed_count += 1;
                    total_size += calculate_dir_size(&path)?;
                }
            }
        }
    }

    // Versión activa (desde symlink actual)
    let active_version = get_active_version(config);

    // Aliases
    let aliases_count = match Aliases::load() {
        Ok(aliases) => aliases.aliases.len(),
        Err(_) => 0,
    };

    // Info de caché
    let cache_info = get_cache_info(config)?;
    let cache_size = get_cache_total_size(config).unwrap_or(0);

    Ok(Stats {
        nvm_version: env!("CARGO_PKG_VERSION").to_string(),
        nvm_location: nvm_dir,
        nvm_size,
        installed_versions_count: installed_count,
        active_version,
        total_node_size: total_size,
        aliases_count,
        cache_size,
        cache_valid: cache_info.is_valid,
        cache_age: cache_info.last_updated,
    })
}

/// Obtener versión activa
fn get_active_version(config: &Config) -> Option<String> {
    let current_link = config.nvm_dir.join("versions").join("current");

    #[cfg(target_os = "windows")]
    {
        // En Windows buscar en el directorio "current"
        if let Ok(_dir) = fs::read_dir(&current_link) {
            // La carpeta actual es un junction, simplemente buscar archivos
            return Some("(active)".to_string());
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // En Unix, leer el symlink
        if let Ok(target) = fs::read_link(&current_link) {
            if let Some(name) = target.file_name() {
                if let Some(name_str) = name.to_str() {
                    return Some(name_str.to_string());
                }
            }
        }
    }

    None
}

/// Calcular tamaño total de un directorio
fn calculate_dir_size(path: &std::path::Path) -> anyhow::Result<u64> {
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

/// Mostrar estadísticas de forma bonita
pub fn display_stats(stats: &Stats) {
    println!("\n{}", "📊 NVM Statistics".bold().cyan());
    println!("{}", "━".repeat(70).bright_black());

    println!("\n{}", "📦 Installation Info:".yellow());
    println!("   {} {}", "Version:".bright_black(), stats.nvm_version);
    println!("   {} {}", "Location:".bright_black(), stats.nvm_location.display());
    println!(
        "   {} {}",
        "Binary size:".bright_black(),
        format_size(stats.nvm_size)
    );

    println!("\n{}", "🔄 Node.js Versions:".yellow());
    println!(
        "   {} {}",
        "Installed:".bright_black(),
        stats.installed_versions_count
    );
    if let Some(active) = &stats.active_version {
        println!("   {} {}", "Active:".bright_black(), active.green());
    } else {
        println!("   {} {}", "Active:".bright_black(), "(none)".bright_black());
    }
    println!(
        "   {} {}",
        "Total size:".bright_black(),
        format_size(stats.total_node_size)
    );

    println!("\n{}", "🏷️  Aliases:".yellow());
    println!("   {} {}", "Defined:".bright_black(), stats.aliases_count);

    println!("\n{}", "💾 Cache:".yellow());
    println!("   {} {}", "Location:".bright_black(), "~/.nvm/cache");
    println!(
        "   {} {}",
        "Size:".bright_black(),
        format_size(stats.cache_size)
    );
    if stats.cache_valid {
        println!("   {} {}", "Status:".bright_black(), "✓ Valid".green());
    } else {
        println!("   {} {}", "Status:".bright_black(), "⚠ Expired".yellow());
    }
    if let Some(age) = stats.cache_age {
        println!("   {} {}", "Age:".bright_black(), format_age(age));
    }

    println!("\n{}", "━".repeat(70).bright_black());
    println!();
}

/// Mostrar estadísticas en formato JSON
pub fn display_stats_json(stats: &Stats) -> anyhow::Result<()> {
    let json = serde_json::json!({
        "nvm_version": stats.nvm_version,
        "nvm_location": stats.nvm_location.to_string_lossy(),
        "nvm_size_bytes": stats.nvm_size,
        "installed_versions": stats.installed_versions_count,
        "active_version": stats.active_version,
        "total_node_size_bytes": stats.total_node_size,
        "aliases_count": stats.aliases_count,
        "cache_size_bytes": stats.cache_size,
        "cache_valid": stats.cache_valid,
        "cache_age_seconds": stats.cache_age,
    });

    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

/// Formato legible de tamaño
fn format_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.2} {}", size, units[unit_idx])
}

/// Formato legible de edad
fn format_age(seconds: u64) -> String {
    if seconds < 60 {
        format!("{} seconds ago", seconds)
    } else if seconds < 3600 {
        format!("{} minutes ago", seconds / 60)
    } else if seconds < 86400 {
        format!("{} hours ago", seconds / 3600)
    } else {
        format!("{} days ago", seconds / 86400)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
    }

    #[test]
    fn test_format_age() {
        assert!(format_age(30).contains("seconds"));
        assert!(format_age(300).contains("minutes"));
        assert!(format_age(7200).contains("hours"));
    }
}
