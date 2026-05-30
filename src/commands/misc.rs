use anyhow::Result;
use crate::t;

use std::io::{self, Write};
use crate::config::Config;
use crate::core::{self, versions, refresh_installed_cache};
use crate::core::nvm_config::NvmConfig;
use crate::i18n::{set_locale, Locale};

/// Remove unused versions (keep current and LTS)
pub async fn cleanup(yes: bool, config: &Config) -> Result<()> {
    println!("{}", t!("cleaning_up"));

    let installed = core::get_installed_versions(config)?;

    if installed.is_empty() {
        println!("{}", t!("no_versions_installed"));
        return Ok(());
    }

    let current_version = versions::get_current_version(config);
    let available_versions = core::get_cached_versions(config).await.unwrap_or_default();

    let mut versions_to_keep = Vec::new();
    let mut versions_to_remove = Vec::new();

    for version in &installed {
        let mut keep = false;

        if let Some(ref current) = current_version {
            if version == current {
                keep = true;
            }
        }

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

    if versions_to_remove.is_empty() {
        println!("{}", t!("no_versions_to_cleanup"));
        return Ok(());
    }

    println!("\n{}", t!("cleanup_title"));
    for version in &versions_to_remove {
        println!("  - {}", version);
    }

    println!("\n{}", t!("cleanup_keeping"));
    if let Some(ref current) = current_version {
        println!("  {} ", t!("cleanup_current_version").replace("{version}", current));
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
        println!("  {}", t!("cleanup_lts_versions").replace("{count}", &lts_count.to_string()));
    }

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

    let mut removed_count = 0;
    for version in &versions_to_remove {
        let version_dir = config.versions_dir().join(version);
        if version_dir.exists() {
            std::fs::remove_dir_all(&version_dir)?;
            removed_count += 1;
        }
    }

    refresh_installed_cache(config)?;

    println!("\n{}", t!("cleanup_complete").replace("{count}", &removed_count.to_string()));

    Ok(())
}

/// Set default version used by `nvm use` when no version or .nvmrc is found
pub fn set_default(version: String, config: &Config) -> Result<()> {
    // Validate: if it's a concrete version, verify it's installed
    let is_concrete = (version.starts_with('v') || !version.contains('/'))
        && version.matches('.').count() == 2;

    if is_concrete {
        let normalized = if version.starts_with('v') {
            version.clone()
        } else {
            format!("v{}", version)
        };
        let version_dir = config.versions_dir().join(&normalized);
        if !version_dir.exists() {
            eprintln!(
                "{}",
                t!("set_default_not_installed")
                    .replace("{}", &normalized)
            );
            eprintln!("Run: nvm install {}", normalized);
            return Ok(());
        }
    }

    let mut nvm_config = NvmConfig::load(&config.config_file());
    nvm_config.default_version = Some(version.clone());
    nvm_config.save(&config.config_file())?;

    println!("{}", t!("set_default_success").replace("{}", &version));
    Ok(())
}

/// Set language/locale and persist it to config
pub fn set_language(locale: String, config: &Config) -> Result<()> {
    if let Some(new_locale) = Locale::from_str(&locale) {
        set_locale(new_locale);

        let mut nvm_config = NvmConfig::load(&config.config_file());
        nvm_config.locale = Some(new_locale.as_str().to_string());
        nvm_config.save(&config.config_file())?;

        println!("{}", t!("locale_persisted").replace("{}", new_locale.as_str()));
    } else {
        println!("{}", t!("unsupported_locale", &locale));
    }
    Ok(())
}

/// Clear the remote and installed version caches
pub fn cache_clear(config: &Config) -> Result<()> {
    let mut cleared = 0u32;

    for path in [config.cache_file(), config.installed_cache_file()] {
        if path.exists() {
            std::fs::remove_file(&path)?;
            cleared += 1;
        }
    }

    if cleared == 0 {
        println!("{}", t!("cache_already_clear"));
    } else {
        println!("{}", t!("cache_cleared").replace("{}", &cleared.to_string()));
    }
    Ok(())
}

/// Enable symlink support on Windows (requires admin)
#[cfg(windows)]
pub fn enable_symlinks() -> Result<()> {
    use std::process::Command;
    use crate::utils::{print_warning, print_success};

    println!("\n{}", t!("enable_symlinks_title"));
    println!("{}", "=".repeat(50));
    println!();

    let is_admin = Command::new("net")
        .args(&["session"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !is_admin {
        print_warning(&t!("enable_symlinks_admin_required"));
        println!();
        println!("Para ejecutar como administrador:");
        println!("  1. Click derecho en PowerShell");
        println!("  2. Selecciona 'Ejecutar como administrador'");
        println!("  3. Ejecuta: nvm enable-symlinks");
        println!();
        return Ok(());
    }

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
    println!("{}", t!("current_version_label").replace("{version}", current_version));

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
