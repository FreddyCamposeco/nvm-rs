use anyhow::{Context, Result};
use crate::t;

use crate::config::Config;
use crate::core::{self, versions, symlink, refresh_installed_cache};


pub async fn uninstall(version: &str, force: bool, config: &Config) -> Result<()> {
    println!("{}", t!("uninstalling_version").replace("{version}", version));

    // Obtener versiones disponibles para resolver aliases
    let available_versions = core::get_cached_versions(config).await?;

    // Resolver la versión (puede ser alias)
    let resolved_version = versions::resolve_version(version, &available_versions)?;

    // Verificar que la versión esté instalada
    let version_dir = config.versions_dir().join(&resolved_version);
    if !version_dir.exists() {
        eprintln!("{}", t!("version_not_installed").replace("{}", &resolved_version));
        return Ok(());
    }

    // Verificar si es la versión actual
    if let Some(current_version) = versions::get_current_version(config) {
        if current_version == resolved_version && !force {
            eprintln!("{}", t!("cannot_uninstall_active")
                .replace("{version}", &resolved_version));
            eprintln!("{}", t!("use_force_to_uninstall"));
            return Ok(());
        }

        // Si es la versión actual y se usa --force, eliminar el symlink
        if current_version == resolved_version && force {
            let current_link = config.current_dir();
            if current_link.exists() {
                symlink::remove_symlink(&current_link)?;
            }
        }
    }

    // Eliminar el directorio de la versión
    std::fs::remove_dir_all(&version_dir)
        .with_context(|| format!("Failed to remove version directory: {}", version_dir.display()))?;

    // Actualizar cache de versiones instaladas
    refresh_installed_cache(config)?;

    println!("{}", t!("version_uninstalled").replace("{version}", &resolved_version));

    Ok(())
}
