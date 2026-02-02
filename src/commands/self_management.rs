use anyhow::Result;
use crate::t;

use std::path::PathBuf;
use std::io::{self, Write};
use crate::core::installer::*;
use crate::core::github::{get_latest_release, get_release_by_tag, get_platform_asset_name, download_asset};


/// Install nvm from GitHub releases
pub async fn install_self(
    version: Option<String>,
    dir: Option<PathBuf>,
    with_self_update: bool,
) -> Result<()> {
    println!("{}", t!("install_self_start"));

    // Determinar versión a instalar
    let release = if let Some(ver) = version {
        if ver == "latest" {
            get_latest_release().await?
        } else {
            get_release_by_tag(&ver).await?
        }
    } else {
        get_latest_release().await?
    };

    println!("{}", t!("install_self_version")
        .replace("{version}", &release.tag_name));

    // Determinar asset apropiado
    let asset_name = get_platform_asset_name(&release.tag_name, with_self_update);
    let asset = release.assets.iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| anyhow::anyhow!(
            "{}", t!("install_self_no_asset")
                .replace("{asset}", &asset_name)
        ))?;

    // Crear directorio temporal
    let temp_dir = std::env::temp_dir().join("nvm-install");
    std::fs::create_dir_all(&temp_dir)?;
    let download_path = temp_dir.join(&asset.name);

    // Descargar binario
    println!("\n{}", t!("downloading"));
    download_asset(asset, &download_path).await?;

    // Verificar checksum (si está disponible)
    println!("{}", t!("install_self_verifying"));
    let checksum = calculate_checksum(&download_path)?;
    println!("SHA256: {}", checksum);

    // Determinar directorio de instalación
    let install_dir = if let Some(d) = dir {
        d
    } else {
        get_install_dir()?
    };

    // Instalar binario
    println!("\n{}", t!("install_self_installing"));
    let installed_path = install_binary(&download_path, &install_dir)?;

    // Limpiar archivos temporales
    let _ = std::fs::remove_dir_all(&temp_dir);

    println!("\n✓ {}", t!("install_self_complete")
        .replace("{path}", &installed_path.display().to_string()));

    // Configurar variables de entorno
    #[cfg(windows)]
    {
        println!("\n{}", t!("install_self_configuring_env"));

        // Configurar NVM_HOME
        let nvm_data_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".nvm");

        if let Err(e) = set_nvm_home(&nvm_data_dir) {
            println!("{}", t!("install_self_env_warning")
                .replace("{error}", &e.to_string()));
        } else {
            println!("✓ {}", t!("install_self_nvm_dir_set")
                .replace("{path}", &nvm_data_dir.display().to_string()));
        }

        // Configurar NVM_BIN
        if let Err(e) = set_nvm_bin(&install_dir) {
            println!("{}", t!("install_self_env_warning")
                .replace("{error}", &e.to_string()));
        } else {
            println!("✓ NVM_BIN configurado ({})", install_dir.display());
        }

        // Configurar NVM_NODE (ruta a current/bin)
        let nvm_node_dir = nvm_data_dir.join("current").join("bin");
        if let Err(e) = set_nvm_node(&nvm_node_dir) {
            println!("{}", t!("install_self_env_warning")
                .replace("{error}", &e.to_string()));
        } else {
            println!("✓ NVM_NODE configurado ({})", nvm_node_dir.display());
        }

        // Agregar nvm/bin al PATH si no está
        if !is_in_path(&install_dir) {
            if let Err(e) = add_to_path(&install_dir) {
                println!("{}", t!("install_self_path_warning")
                    .replace("{error}", &e.to_string()));
                println!("\n{}", get_path_instructions(&install_dir));
            } else {
                println!("✓ {}", t!("install_self_path_set"));
            }
        } else {
            println!("✓ {}", t!("install_self_path_already_set"));
        }

        // Agregar nvm/current/bin al PATH para la versión activa
        let current_dir = nvm_data_dir.join("current").join("bin");
        if !is_in_path(&current_dir) {
            if let Err(e) = add_to_path(&current_dir) {
                println!("{}", t!("install_self_path_warning")
                    .replace("{error}", &e.to_string()));
            } else {
                println!("✓ PATH configurado para versión activa ({})", current_dir.display());
            }
        } else {
            println!("✓ PATH de versión activa ya configurado");
        }

        println!("\n{}", t!("install_self_restart_terminal"));
    }

    // Configurar variables de entorno en Unix (shell config)
    #[cfg(not(windows))]
    {
        println!("\n{}", t!("install_self_configuring_env"));

        let nvm_data_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".nvm");

        if let Err(e) = set_nvm_dir(&nvm_data_dir) {
            println!("{}", t!("install_self_env_warning")
                .replace("{error}", &e.to_string()));
            println!("\n{}", get_path_instructions(&install_dir));
        } else {
            println!("✓ {}", t!("install_self_nvm_dir_set")
                .replace("{path}", &nvm_data_dir.display().to_string()));
        }

        let nvm_bin_dir = nvm_data_dir.join("bin");
        if !is_in_path(&nvm_bin_dir) {
            if let Err(e) = add_to_path(&nvm_bin_dir) {
                println!("{}", t!("install_self_path_warning")
                    .replace("{error}", &e.to_string()));
                println!("\n{}", get_path_instructions(&install_dir));
            } else {
                println!("✓ {}", t!("install_self_path_set"));
            }
        } else {
            println!("✓ {}", t!("install_self_path_already_set"));
        }

        println!("\n{}", t!("install_self_restart_terminal"));
    }

    Ok(())
}

/// Uninstall nvm from the system
pub fn uninstall_self(
    dir: Option<PathBuf>,
    yes: bool,
    purge: bool,
    remove_config: bool,
) -> Result<()> {
    println!("{}", t!("uninstall_self_start"));

    // Determinar directorio
    let install_dir = if let Some(d) = dir {
        d
    } else {
        get_install_dir()?
    };

    #[cfg(windows)]
    let exe_path = install_dir.join("nvm.exe");
    #[cfg(not(windows))]
    let exe_path = install_dir.join("nvm");

    if !exe_path.exists() {
        println!("{}", t!("uninstall_self_not_found")
            .replace("{path}", &exe_path.display().to_string()));
        return Ok(());
    }

    // Confirmación
    if !yes {
        let mut msg = t!("uninstall_self_confirm")
            .replace("{path}", &exe_path.display().to_string());
        if purge {
            msg.push_str("\n⚠️  --purge: This will remove ALL Node.js versions and data");
        }
        if remove_config {
            msg.push_str("\n⚠️  --remove-config: This will remove shell configuration");
        }
        print!("\n{} ", msg);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" && input != "s" && input != "si" {
            println!("{}", t!("uninstall_self_cancelled"));
            return Ok(());
        }
    }

    // Usar limpieza completa
    let data_dir = dirs::home_dir()
        .map(|h| h.join(".nvm"));

    full_uninstall_cleanup(Some(&install_dir), data_dir.as_deref(), purge, remove_config)?;

    Ok(())
}

/// Update nvm to the latest version
pub async fn update_self(version: Option<String>, with_self_update: bool) -> Result<()> {
    println!("{}", t!("update_self_start"));

    // Obtener ejecutable actual
    let current_exe = get_current_executable()?;
    let install_dir = current_exe.parent()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine installation directory"))?;

    // Determinar versión a instalar
    let release = if let Some(ver) = version {
        if ver == "latest" {
            get_latest_release().await?
        } else {
            get_release_by_tag(&ver).await?
        }
    } else {
        get_latest_release().await?
    };

    println!("{}", t!("update_self_version")
        .replace("{version}", &release.tag_name));

    // Determinar asset apropiado
    let asset_name = get_platform_asset_name(&release.tag_name, with_self_update);
    let asset = release.assets.iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| anyhow::anyhow!(
            "{}", t!("install_self_no_asset")
                .replace("{asset}", &asset_name)
        ))?;

    // Crear directorio temporal
    let temp_dir = std::env::temp_dir().join("nvm-update");
    std::fs::create_dir_all(&temp_dir)?;
    let download_path = temp_dir.join(&asset.name);

    // Descargar binario
    println!("\n{}", t!("downloading"));
    download_asset(asset, &download_path).await?;

    // Verificar checksum
    println!("{}", t!("install_self_verifying"));
    let checksum = calculate_checksum(&download_path)?;
    println!("SHA256: {}", checksum);

    // Actualizar binario
    println!("\n{}", t!("update_self_installing"));
    let _installed_path = install_binary(&download_path, install_dir)?;

    // Limpiar archivos temporales
    let _ = std::fs::remove_dir_all(&temp_dir);

    println!("\n✓ {}", t!("update_self_complete")
        .replace("{version}", &release.tag_name));
    println!("\n{}", t!("restart_required"));

    Ok(())
}
