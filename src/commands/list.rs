use anyhow::Result;
use crate::t;

use std::collections::BTreeMap;
use crate::config::Config;
use crate::core::{self, versions};


/// List installed Node.js versions
pub async fn list_installed(config: &Config) -> Result<()> {
    // Obtener versiones instaladas
    let mut installed = core::get_installed_versions(config)?;

    if installed.is_empty() {
        println!("{}", t!("no_versions_installed"));
        return Ok(());
    }

    // Ordenar versiones
    versions::sort_versions(&mut installed);

    // Obtener versión actual
    let current = versions::get_current_version(config);

    // Obtener información de versiones remotas para LTS info
    let available_versions = core::get_cached_versions(config).await.unwrap_or_default();

    // Mostrar título
    println!("\n{}", t!("installed_versions"));

    // Mostrar cada versión
    for version in &installed {
        let is_current = current.as_ref().map(|c| c == version).unwrap_or(false);
        let formatted = versions::format_installed_version(version, is_current, &available_versions);
        println!("{}", formatted);
    }

    // Mostrar versión actual al final
    if let Some(curr) = current {
        println!("\n{} {}", t!("current_label"), curr);
    }

    Ok(())
}

/// List remote versions available for download
pub async fn list_remote(lts: bool, config: &Config) -> Result<()> {
    use versions::VersionFilter;

    println!("{}", t!("fetching_versions"));

    // Get cached versions
    match core::get_cached_versions(config).await {
        Ok(versions_list) => {
            // Apply filters
            let mut filter = VersionFilter::new();
            if lts {
                filter = filter.lts_only();
            }

            // Detect platform for filtering
            #[cfg(target_os = "windows")]
            let platform = "win";
            #[cfg(target_os = "macos")]
            let platform = "darwin";
            #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
            let platform = "linux";

            filter = filter.platform(platform.to_string());

            let filtered_versions = filter.apply(versions_list.clone());

            // Get installed versions
            let installed = core::get_installed_versions(config).unwrap_or_default();

            // Group versions by major version
            let mut groups: BTreeMap<u32, Vec<_>> = BTreeMap::new();
            for version in &filtered_versions {
                // Extract major version (e.g., "v24.11.1" -> 24)
                if let Some(major_str) = version.version.strip_prefix('v') {
                    if let Some(major) = major_str.split('.').next() {
                        if let Ok(major_num) = major.parse::<u32>() {
                            groups.entry(major_num).or_insert_with(Vec::new).push(version);
                        }
                    }
                }
            }

            // Show filter info
            if lts {
                println!("\n{}", t!("showing_lts_versions"));
            }

            println!(
                "\n{} {} {} {}:",
                t!("showing"),
                filtered_versions.len(),
                t!("of"),
                versions_list.len()
            );

            // Display grouped versions
            for (major, mut major_versions) in groups.into_iter().rev() {
                // Sort versions in descending order
                major_versions.sort_by(|a, b| b.version.cmp(&a.version));

                println!("\n v{} releases:", major);

                for version in major_versions {
                    let is_installed = installed.contains(&version.version);
                    let status = if is_installed { "[installed]" } else { "" };

                    if let Some(lts_name) = version.lts.name() {
                        println!("  {} (LTS: {}) {}", version.version, lts_name, status);
                    } else {
                        println!("  {} {}", version.version, status);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}: {}", t!("error_fetching_versions"), e);
        }
    }

    Ok(())
}

/// Show current active version
pub fn show_current(config: &Config) -> Result<()> {
    if let Some(current_version) = versions::get_current_version(config) {
        println!("{}", current_version);
    } else {
        println!("{}", t!("no_current_version"));
    }

    Ok(())
}
