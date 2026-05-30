use anyhow::Result;
use crate::t;

use crate::config::Config;
use crate::core::{download, extract, versions, refresh_installed_cache};


pub async fn install(version: &str, no_use: bool, config: &Config) -> Result<()> {
    println!("{} {}", t!("installing_node"), version);

    let available_versions = crate::core::get_cached_versions(config).await?;
    let resolved_version = versions::resolve_version(version, &available_versions)?;

    let node_version = available_versions
        .iter()
        .find(|v| v.version == resolved_version)
        .ok_or_else(|| anyhow::anyhow!("Version {} not found", resolved_version))?;

    println!("Resolved to: {}", node_version.version);

    let version_dir = config.versions_dir().join(&node_version.version);
    if version_dir.exists() {
        println!("{}", t!("version_already_installed").replace("{}", &node_version.version));
        return Ok(());
    }

    let download_dir = config.nvm_dir.join("cache");
    std::fs::create_dir_all(&download_dir)?;

    println!("\n{}", t!("downloading"));
    let archive_path = download::download_node_archive(node_version, &download_dir, config).await?;

    let versions_dir = config.versions_dir();
    std::fs::create_dir_all(&versions_dir)?;

    println!("\n{}", t!("extracting"));
    let extracted_path = extract::extract_archive(&archive_path, &versions_dir)?;

    if extracted_path != version_dir {
        extract::move_extracted_files(&extracted_path, &version_dir)?;
    }

    refresh_installed_cache(config)?;
    std::fs::remove_file(&archive_path)?;

    println!("\n✓ {}", t!("installed_node").replace("{}", &node_version.version));

    if !no_use {
        println!("\n{}", t!("install_switching").replace("{}", &node_version.version));
        if let Err(e) = super::use_version::use_version(
            Some(node_version.version.clone()),
            config,
        ).await {
            eprintln!("⚠ {}: {}", t!("install_switch_failed"), e);
            eprintln!("{}", t!("install_use_hint").replace("{}", &node_version.version));
        }
    }

    Ok(())
}
