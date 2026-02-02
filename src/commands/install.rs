use anyhow::Result;
use crate::t;

use crate::config::Config;
use crate::core::{download, extract, versions, refresh_installed_cache};


pub async fn install(version: &str, config: &Config) -> Result<()> {
    println!("{} {}", t!("installing_node"), version);

    // Obtener lista de versiones disponibles
    let available_versions = crate::core::get_cached_versions(config).await?;

    // Resolver la versión (puede ser un alias como "latest" o "lts")
    let resolved_version = versions::resolve_version(version, &available_versions)?;

    // Buscar la versión completa en la lista
    let node_version = available_versions
        .iter()
        .find(|v| v.version == resolved_version)
        .ok_or_else(|| anyhow::anyhow!("Version {} not found", resolved_version))?;

    println!("Resolved to: {}", node_version.version);

    // Verificar si ya está instalada
    let version_dir = config.versions_dir().join(&node_version.version);
    if version_dir.exists() {
        println!("{}", t!("version_already_installed").replace("{}", &node_version.version));
        return Ok(());
    }

    // Crear directorio temporal para descargas
    let download_dir = config.nvm_dir.join("cache");
    std::fs::create_dir_all(&download_dir)?;

    // Descargar archivo
    println!("\n{}", t!("downloading"));
    let archive_path = download::download_node_archive(node_version, &download_dir, config).await?;

    // Crear directorio de versiones si no existe
    let versions_dir = config.versions_dir();
    std::fs::create_dir_all(&versions_dir)?;

    // Extraer archivo
    println!("\n{}", t!("extracting"));
    let extracted_path = extract::extract_archive(&archive_path, &versions_dir)?;

    // Mover al directorio final (versión normalizada)
    if extracted_path != version_dir {
        extract::move_extracted_files(&extracted_path, &version_dir)?;
    }

    // Actualizar cache de versiones instaladas
    refresh_installed_cache(config)?;

    // Eliminar archivo de descarga
    std::fs::remove_file(&archive_path)?;

    println!("\n✓ {}", t!("installed_node").replace("{}", &node_version.version));

    Ok(())
}
