use crate::config::Config;
use crate::utils::*;
use crate::t;
use std::fs;

pub fn install_node(config: &Config, version: &str) -> anyhow::Result<()> {
    let resolved_version = crate::versions::resolve_version(version)?;
    let version_dir = config.nvm_dir.join("versions").join(&resolved_version);

    if version_dir.exists() {
        return Err(anyhow::anyhow!("{}", t!("version_already_installed", &resolved_version)));
    }

    print_info(&t!("installing_node", &resolved_version));

    // Create version directory
    fs::create_dir_all(&version_dir)?;

    // For MVP, just create a dummy file to simulate installation
    let dummy_file = version_dir.join("node.exe");
    fs::write(&dummy_file, b"dummy node binary")?;

    print_success(&t!("installed_node", &resolved_version));

    Ok(())
}

pub fn uninstall_node(config: &Config, version: &str, force: bool) -> anyhow::Result<()> {
    let resolved_version = crate::versions::resolve_version(version)?;
    let version_dir = config.nvm_dir.join("versions").join(&resolved_version);

    if !version_dir.exists() {
        return Err(anyhow::anyhow!("{}", t!("version_not_installed", &resolved_version)));
    }

    if !force {
        // Check if it's current version
        let current_path = config.nvm_dir.join("current");
        if current_path.exists() {
            #[cfg(unix)]
            {
                if let Ok(target) = fs::read_link(&current_path) {
                    if target == version_dir {
                        return Err(anyhow::anyhow!("{}", t!("cannot_uninstall_current")));
                    }
                }
            }
            #[cfg(windows)]
            {
                // On Windows, current might be a copied directory
                // For simplicity, we'll allow uninstall but warn
                print_info(&t!("uninstalling_node"));
            }
        }
    }

    fs::remove_dir_all(&version_dir)?;
    print_success(&t!("uninstalled_node", &resolved_version));

    Ok(())
}
