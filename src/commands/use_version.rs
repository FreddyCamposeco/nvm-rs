use anyhow::{Context, Result};
use crate::t;

use crate::config::Config;
use crate::core::{self, symlink, versions};
use crate::core::nvm_config::NvmConfig;


pub async fn use_version(version: Option<String>, config: &Config) -> Result<()> {
    let version_to_use = if let Some(ver) = version {
        ver
    } else {
        // 1. Check .nvmrc in directory tree
        if let Some((nvmrc_path, nvmrc_version)) = versions::find_nvmrc_in_tree(None) {
            println!("Found .nvmrc: {}", nvmrc_path.display());
            println!("Using version from .nvmrc: {}", nvmrc_version);
            nvmrc_version
        } else {
            // 2. Check default_version from persisted config
            let nvm_config = NvmConfig::load(&config.config_file());
            if let Some(default) = nvm_config.default_version {
                println!("{}", t!("use_default_version").replace("{}", &default));
                default
            } else {
                eprintln!("{}", t!("use_no_version_specified"));
                eprintln!("Usage: nvm use <version>");
                eprintln!("{}", t!("use_set_default_hint"));
                return Ok(());
            }
        }
    };

    println!("Switching to Node.js {}...", version_to_use);

    let available_versions = core::get_cached_versions(config).await?;
    let resolved_version = versions::resolve_version(&version_to_use, &available_versions)?;

    let version_dir = config.versions_dir().join(&resolved_version);
    if !version_dir.exists() {
        eprintln!("{}", t!("version_not_installed").replace("{}", &resolved_version));
        eprintln!("Run: nvm install {}", resolved_version);
        return Ok(());
    }

    let current_link = config.current_dir();
    println!("{}", t!("creating_symlink"));

    #[cfg(windows)]
    let symlink_target = &version_dir;

    #[cfg(not(windows))]
    let symlink_target = &version_dir.join("bin");

    symlink::create_or_update_symlink(symlink_target, &current_link)
        .context("Failed to create symlink")?;

    // Persist active version to ~/.nvm/.nvm-version (not through the symlink)
    let version_file = config.version_file();
    std::fs::write(&version_file, &resolved_version)
        .context("Failed to persist current version")?;

    println!("\n✓ {}", t!("now_using_node").replace("{}", &resolved_version));

    if symlink::is_valid_symlink(&current_link) {
        println!("{}", t!("symlink_created"));
    }

    Ok(())
}
