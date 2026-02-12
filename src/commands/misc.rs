use anyhow::Result;
use crate::t;

use std::io::{self, Write};
use crate::config::Config;
use crate::core::{self, versions, refresh_installed_cache};
use crate::i18n::{set_locale, Locale};

/// Remove unused versions (keep current and LTS)
pub async fn cleanup(yes: bool, config: &Config) -> Result<()> {
    println!("{}", t!("cleaning_up"));

    // Obtener versiones instaladas
    let installed = core::get_installed_versions(config)?;

    if installed.is_empty() {
        println!("{}", t!("no_versions_installed"));
        return Ok(());
    }

    // Obtener versión actual
    let current_version = versions::get_current_version(config);

    // Obtener información de versiones remotas para identificar LTS
    let available_versions = core::get_cached_versions(config).await.unwrap_or_default();

    // Determinar qué versiones mantener
    let mut versions_to_keep = Vec::new();
    let mut versions_to_remove = Vec::new();

    for version in &installed {
        let mut keep = false;

        // Mantener versión actual
        if let Some(ref current) = current_version {
            if version == current {
                keep = true;
            }
        }

        // Mantener versiones LTS
        if let Some(node_version) = available_versions.iter().find(|v| &v.version == version) {
            if node_version.lts.is_lts() {
                keep = true;
            }
        }

        if keep {
            versions_to_keep.push(version.clone());
        } else {
            versions_to_remove.push(version.clone());
        }
    }

    // Si no hay nada que eliminar
    if versions_to_remove.is_empty() {
        println!("{}", t!("no_versions_to_cleanup"));
        return Ok(());
    }

    // Mostrar información
    println!("\n{}", t!("cleanup_title"));
    for version in &versions_to_remove {
        println!("  - {}", version);
    }

    println!("\n{}", t!("cleanup_keeping"));
    if let Some(ref current) = current_version {
        println!("  {} ", t!("cleanup_current_version")
            .replace("{version}", current));
    }
    let lts_count = versions_to_keep.iter()
        .filter(|v| {
            available_versions.iter()
                .find(|av| &av.version == *v)
                .map(|av| av.lts.is_lts())
                .unwrap_or(false)
        })
        .count();
    if lts_count > 0 {
        println!("  {}", t!("cleanup_lts_versions")
            .replace("{count}", &lts_count.to_string()));
    }

    // Confirmación (si no se usa --yes)
    if !yes {
        print!("\n{}", t!("cleanup_confirm"));
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" && input != "s" && input != "si" {
            println!("{}", t!("cleanup_cancelled"));
            return Ok(());
        }
    }

    // Eliminar versiones
    let mut removed_count = 0;
    for version in &versions_to_remove {
        let version_dir = config.versions_dir().join(version);
        if version_dir.exists() {
            std::fs::remove_dir_all(&version_dir)?;
            removed_count += 1;
        }
    }

    // Actualizar cache
    refresh_installed_cache(config)?;

    println!("\n{}", t!("cleanup_complete")
        .replace("{count}", &removed_count.to_string()));

    Ok(())
}

/// Set default version for new shells (placeholder)
pub fn set_default(version: String) -> Result<()> {
    println!("Command 'set-default {}' - Not yet implemented", version);
    println!("This will be implemented in Phase 7 of the migration plan");
    Ok(())
}

/// Set language/locale
pub fn set_language(locale: String) -> Result<()> {
    if let Some(new_locale) = Locale::from_str(&locale) {
        set_locale(new_locale);
        println!("{}", t!("locale_set", new_locale.as_str()));
    } else {
        println!("{}", t!("unsupported_locale", &locale));
    }
    Ok(())
}

/// Enable symlink support on Windows (requires admin)
#[cfg(windows)]
pub fn enable_symlinks() -> Result<()> {
    use std::process::Command;

    println!("\n{}", t!("enable_symlinks_title"));
    println!("{}", "=".repeat(50));
    println!();

    // Verificar si se ejecuta como administrador
    let output = if let Ok(output) = Command::new("net")
        .args(&["session"])
        .output() {
        output.status.success()
    } else {
        false
    };

    if !output {
        print_warning(&t!("enable_symlinks_admin_required"));
        println!();
        println!("Para ejecutar como administrador:");
        println!("  1. Click derecho en PowerShell");
        println!("  2. Selecciona 'Ejecutar como administrador'");
        println!("  3. Ejecuta: nvm enable-symlinks");
        println!();
        return Ok(());
    }

    // Intentar habilitar Developer Mode
    println!("Intentando habilitar soporte de symlinks...");
    println!();

    let dev_mode_key = "HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AppModelUnlock";
    let result = Command::new("reg")
        .args(&["add", dev_mode_key, "/v", "AllowDevelopmentWithoutDevLicense", "/t", "REG_DWORD", "/d", "1", "/f"])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            print_success(&t!("enable_symlinks_success"));
            println!();
            println!("Cambios aplicados:");
            println!("  • Developer Mode habilitado en el registro");
            println!();
            println!("Próximos pasos:");
            println!("  1. Reinicia tu PC para aplicar los cambios");
            println!("  2. Ejecuta 'nvm doctor' para verificar");
            println!();
        }
        _ => {
            print_warning(&t!("enable_symlinks_failed"));
            println!();
            println!("Habilita manualmente:");
            println!("  1. Configuración > Actualización y seguridad");
            println!("  2. Para desarrolladores");
            println!("  3. Activa 'Modo de desarrollador'");
            println!();
        }
    }

    Ok(())
}

/// Self-update using self_update crate (if feature enabled)
#[cfg(feature = "self-update")]
pub fn self_update() -> Result<()> {
    use self_update::cargo_crate_version;

    println!("{}", t!("checking_for_updates"));

    let current_version = cargo_crate_version!();
    println!("{}", t!("current_version_label")
        .replace("{version}", current_version));

    // Configurar el actualizador
    let status = self_update::backends::github::Update::configure()
        .repo_owner("FreddyCamposeco")
        .repo_name("nvm-rs")
        .bin_name("nvm")
        .current_version(current_version)
        .show_download_progress(true)
        .no_confirm(false)
        .build()?
        .update()?;

    match status {
        self_update::Status::UpToDate(v) => {
            println!("{}", t!("already_latest"));
            println!("Version: {}", v);
        }
        self_update::Status::Updated(v) => {
            println!("{}", t!("update_complete"));
            println!("{}", t!("updating_to").replace("{version}", &v));
            println!("\n{}", t!("restart_required"));
        }
    }

    Ok(())
}
