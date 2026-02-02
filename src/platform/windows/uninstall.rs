use crate::error::{with_context, Result};
use std::fs;
use std::path::Path;

pub fn full_uninstall_cleanup(
    install_dir: Option<&Path>,
    data_dir: Option<&Path>,
    purge: bool,
    remove_config: bool,
) -> Result<()> {
    use std::ptr;
    use winapi::um::winuser::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    println!("🔄 Desinstalando nvm...");

    // 1. Eliminar binario
    let install_dir = if let Some(d) = install_dir {
        d.to_path_buf()
    } else {
        crate::core::installer::get_install_dir()?
    };

    let exe_path = install_dir.join("nvm.exe");
    if exe_path.exists() {
        fs::remove_file(&exe_path)
            .map_err(|e| with_context("Failed to remove nvm.exe", e))?;
        println!("✓ Binario nvm.exe eliminado");
    }

    // 2. Eliminar del PATH (NVM_BIN) si remove_config
    if remove_config {
        if let Err(e) = crate::platform::windows::env::remove_from_path(&install_dir) {
            eprintln!("⚠ No se pudo remover NVM_BIN del PATH: {}", e);
        } else {
            println!("✓ NVM_BIN removido del PATH");
        }
    }

    // 3. Eliminar del PATH (NVM_NODE/bin si existe) si remove_config
    if remove_config {
        if let Some(data_d) = data_dir {
            let node_bin = data_d.join("current").join("bin");
            if node_bin.exists() {
                if let Err(e) = crate::platform::windows::env::remove_from_path(&node_bin) {
                    eprintln!("⚠ No se pudo remover Node bin del PATH: {}", e);
                } else {
                    println!("✓ Node bin removido del PATH: {}", node_bin.display());
                }
            }
        }
    }

    // 4. Eliminar todas las variables de entorno si remove_config
    if remove_config {
        let env_vars = vec!["NVM_HOME", "NVM_BIN", "NVM_NODE", "NODE_MIRROR"];
        for var in env_vars {
            let cmd = format!(
                "[Environment]::SetEnvironmentVariable('{}', $null, 'User')",
                var
            );
            let status = std::process::Command::new("powershell")
                .args(["-NoProfile", "-Command", &cmd])
                .status()
                .map_err(|e| with_context(&format!("Failed to remove {}", var), e))?;

            if status.success() {
                println!("✓ Variable {} eliminada", var);
            } else {
                eprintln!("⚠ No se pudo eliminar variable {}", var);
            }
        }
    }

    // 5. Notificar al sistema del cambio de variables si remove_config
    if remove_config {
        unsafe {
            let param = "Environment\0".encode_utf16().collect::<Vec<u16>>();
            SendMessageTimeoutW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                0,
                param.as_ptr() as isize,
                SMTO_ABORTIFHUNG,
                5000,
                ptr::null_mut(),
            );
        }
    }

    // 6. Eliminar directorio de datos si --purge
    if purge {
        if let Some(data_d) = data_dir {
            if data_d.exists() {
                println!("🗑️  Eliminando directorio de datos: {}", data_d.display());
                match fs::remove_dir_all(data_d) {
                    Ok(_) => println!("✓ Directorio de datos eliminado (versiones de Node.js, cache, aliases)"),
                    Err(e) => eprintln!("⚠ No se pudo eliminar directorio {}: {}", data_d.display(), e),
                }
            }
        }
    }

    println!("\n✅ nvm ha sido completamente desinstalado");
    if !remove_config {
        println!("💡 Para remover configuración del PATH y variables, usa: nvm uninstall-self --remove-config");
    }
    if !purge {
        println!("💡 Para eliminar todas las versiones de Node.js instaladas, usa: nvm uninstall-self --purge");
    }
    if remove_config {
        println!("💡 Reinicia tu terminal para aplicar todos los cambios");
    }

    Ok(())
}
