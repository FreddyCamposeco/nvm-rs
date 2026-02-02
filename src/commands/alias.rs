use anyhow::Result;
use crate::t;

use colored::Colorize;
use crate::config::Config;
use crate::core::{self, aliases::Aliases, versions};


/// Create or update an alias
pub async fn create_alias(name: String, version: String, config: &Config) -> Result<()> {
    // Validar nombre de alias
    if !Aliases::is_valid_alias_name(&name) {
        eprintln!("{}", t!("invalid_alias_name").replace("{name}", &name));
        eprintln!("{}", t!("invalid_alias_name_help"));
        return Ok(());
    }

    // Obtener versiones disponibles para resolver aliases
    let available_versions = core::get_cached_versions(config).await?;

    // Resolver la versión (puede ser otro alias o versión directa)
    let resolved_version = versions::resolve_version(&version, &available_versions)?;

    // Verificar que la versión exista (instalada o disponible)
    let version_dir = config.versions_dir().join(&resolved_version);
    if !version_dir.exists() {
        // Si no está instalada, verificar que al menos esté disponible
        let version_exists = available_versions.iter()
            .any(|v| v.version == resolved_version);

        if !version_exists {
            eprintln!("{}", t!("version_not_found").replace("{}", &resolved_version));
            return Ok(());
        }
    }

    // Cargar aliases existentes
    let mut aliases = Aliases::load()?;

    // Verificar si es actualización o creación
    let is_update = aliases.contains(&name);

    // Establecer el alias
    aliases.set(name.clone(), resolved_version.clone());

    // Guardar aliases
    aliases.save()?;

    // Mostrar mensaje apropiado
    if is_update {
        println!("{}", t!("alias_updated")
            .replace("{name}", &name)
            .replace("{version}", &resolved_version));
    } else {
        println!("{}", t!("alias_created")
            .replace("{name}", &name)
            .replace("{version}", &resolved_version));
    }

    Ok(())
}

/// Remove an alias
pub fn remove_alias(name: String) -> Result<()> {
    // Cargar aliases existentes
    let mut aliases = Aliases::load()?;

    // Intentar eliminar el alias
    if let Some(removed_version) = aliases.remove(&name) {
        // Guardar cambios
        aliases.save()?;

        println!("{}", t!("alias_removed").replace("{name}", &name));
        println!("  {} -> {}", name, removed_version);
    } else {
        eprintln!("{}", t!("alias_not_found").replace("{name}", &name));
    }

    Ok(())
}

/// List all defined aliases
pub fn list_aliases() -> Result<()> {
    // Cargar aliases
    let aliases = Aliases::load()?;

    // Verificar si hay aliases definidos
    if aliases.aliases.is_empty() {
        println!("{}", t!("no_aliases_defined"));
        return Ok(());
    }

    // Mostrar título
    println!("\n{}", t!("aliases_list_title"));

    // Obtener y mostrar aliases ordenados
    let list = aliases.list();
    for (name, version) in list {
        println!("  {} -> {}", name.cyan().bold(), version);
    }

    println!();

    Ok(())
}
