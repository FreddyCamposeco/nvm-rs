use crate::error::{with_context, Result};
use std::fs;
use std::path::Path;

pub fn full_uninstall_cleanup(
    install_dir: Option<&Path>,
    data_dir: Option<&Path>,
    purge: bool,
    remove_config: bool,
) -> Result<()> {
    // En Unix, la limpieza es más simple
    if let Some(d) = install_dir {
        let exe_path = d.join("nvm");
        if exe_path.exists() {
            fs::remove_file(&exe_path)
                .map_err(|e| with_context("Failed to remove nvm", e))?;
            println!("✓ nvm binary removed");
        }
    }

    // Eliminar configuración del shell si --remove-config
    if remove_config {
        if let Ok((config_path, _)) = crate::platform::unix::shell::detect_shell_config() {
            if let Err(e) = crate::platform::unix::shell::remove_shell_block(&config_path) {
                eprintln!("⚠ Could not remove shell configuration: {}", e);
            } else {
                println!("✓ Shell configuration removed from {}", config_path.display());
            }
        }
    }

    // Eliminar directorio de datos si --purge
    if purge {
        if let Some(data_d) = data_dir {
            if data_d.exists() {
                println!("🗑️  Removing data directory: {}", data_d.display());
                if let Err(e) = fs::remove_dir_all(data_d) {
                    eprintln!("⚠ Could not remove data directory: {}", e);
                } else {
                    println!("✓ Data directory removed (Node.js versions, cache, aliases)");
                }
            }
        }
    }

    println!("✅ nvm uninstalled");
    if !remove_config {
        println!("💡 To remove shell configuration, use: nvm uninstall-self --remove-config");
    }
    if !purge {
        println!("💡 To remove all installed Node.js versions, use: nvm uninstall-self --purge");
    }
    Ok(())
}
