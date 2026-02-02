use anyhow::{Context, Result};
use crate::t;

use crate::config::Config;
use crate::core::{self, symlink, versions};


pub async fn use_version(version: Option<String>, config: &Config) -> Result<()> {
    // Determinar qué versión usar
    let version_to_use = if let Some(ver) = version {
        ver
    } else {
        // Si no se especificó versión, buscar .nvmrc
        if let Some((nvmrc_path, nvmrc_version)) = versions::find_nvmrc_in_tree(None) {
            println!("Found .nvmrc: {}", nvmrc_path.display());
            println!("Using version from .nvmrc: {}", nvmrc_version);
            nvmrc_version
        } else {
            eprintln!("No version specified and no .nvmrc found");
            eprintln!("Usage: nvm use <version>");
            return Ok(());
        }
    };

    println!("Switching to Node.js {}...", version_to_use);

    // Obtener versiones disponibles para resolver aliases
    let available_versions = core::get_cached_versions(config).await?;

    // Resolver la versión (puede ser alias)
    let resolved_version = versions::resolve_version(&version_to_use, &available_versions)?;

    // Verificar que la versión esté instalada
    let version_dir = config.versions_dir().join(&resolved_version);
    if !version_dir.exists() {
        eprintln!("{}", t!("version_not_installed").replace("{}", &resolved_version));
        eprintln!("Run: nvm install {}", resolved_version);
        return Ok(());
    }

    // Crear o actualizar symlink
    let current_link = config.current_dir();
    println!("{}", t!("creating_symlink"));

    // En Windows, el symlink apunta a la raíz de la versión
    // En Unix, debe apuntar a la subcarpeta bin/
    #[cfg(windows)]
    let symlink_target = &version_dir;

    #[cfg(not(windows))]
    let symlink_target = &version_dir.join("bin");

    symlink::create_or_update_symlink(symlink_target, &current_link)
        .context("Failed to create symlink")?;

    // Persistir la versión en .nvm-version para recuperación confiable
    symlink::persist_current_version(&current_link, &resolved_version)
        .context("Failed to persist current version")?;

    println!("\n✓ {}", t!("now_using_node").replace("{}", &resolved_version));

    // Mostrar información adicional
    if symlink::is_valid_symlink(&current_link) {
        println!("{}", t!("symlink_created"));
    }

    Ok(())
}
