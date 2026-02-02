use crate::error::{message, with_context, Result};
use std::path::Path;

/// Agrega el directorio al PATH del usuario (permanente)
pub fn add_to_path(install_dir: &Path) -> Result<()> {
    use std::ptr;
    use winapi::um::winuser::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    // Obtener PATH actual del usuario
    let current_path = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "[Environment]::GetEnvironmentVariable('Path', 'User')",
        ])
        .output()
        .map_err(|e| with_context("Failed to get current PATH", e))?;

    let current_path_str = String::from_utf8_lossy(&current_path.stdout)
        .trim()
        .to_string();
    let install_dir_str = install_dir.to_string_lossy();

    // Verificar si ya está en el PATH
    if current_path_str
        .split(';')
        .any(|p| p.trim() == install_dir_str.as_ref())
    {
        return Ok(());
    }

    // Agregar al PATH
    let new_path = if current_path_str.is_empty() {
        install_dir_str.to_string()
    } else {
        format!("{};{}", current_path_str, install_dir_str)
    };

    // Establecer la nueva variable PATH
    let status = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "[Environment]::SetEnvironmentVariable('Path', '{}', 'User')",
                new_path
            ),
        ])
        .status()
        .map_err(|e| with_context("Failed to set PATH", e))?;

    if !status.success() {
        return Err(message("Failed to update PATH environment variable"));
    }

    // Notificar al sistema del cambio
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

    Ok(())
}

/// Elimina el directorio del PATH del usuario
pub fn remove_from_path(install_dir: &Path) -> Result<()> {
    use std::ptr;
    use winapi::um::winuser::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    // Obtener PATH actual del usuario
    let current_path = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "[Environment]::GetEnvironmentVariable('Path', 'User')",
        ])
        .output()
        .map_err(|e| with_context("Failed to get current PATH", e))?;

    let current_path_str = String::from_utf8_lossy(&current_path.stdout)
        .trim()
        .to_string();
    let install_dir_str = install_dir.to_string_lossy();

    // Filtrar el directorio de instalación
    let new_path: Vec<&str> = current_path_str
        .split(';')
        .filter(|p| p.trim() != install_dir_str.as_ref())
        .collect();

    let new_path = new_path.join(";");

    // Establecer la nueva variable PATH
    let status = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "[Environment]::SetEnvironmentVariable('Path', '{}', 'User')",
                new_path
            ),
        ])
        .status()
        .map_err(|e| with_context("Failed to set PATH", e))?;

    if !status.success() {
        return Err(message("Failed to update PATH environment variable"));
    }

    // Notificar al sistema del cambio
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

    Ok(())
}

/// Establece la variable de entorno NVM_HOME
pub fn set_nvm_home(nvm_dir: &Path) -> Result<()> {
    use std::ptr;
    use winapi::um::winuser::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    let nvm_dir_str = nvm_dir.to_string_lossy();

    // Establecer NVM_HOME
    let status = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "[Environment]::SetEnvironmentVariable('NVM_HOME', '{}', 'User')",
                nvm_dir_str
            ),
        ])
        .status()
        .map_err(|e| with_context("Failed to set NVM_HOME", e))?;

    if !status.success() {
        return Err(message("Failed to set NVM_HOME environment variable"));
    }

    // Notificar al sistema del cambio
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

    Ok(())
}

/// Establece la variable de entorno NVM_BIN
pub fn set_nvm_bin(nvm_bin: &Path) -> Result<()> {
    use std::ptr;
    use winapi::um::winuser::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    let nvm_bin_str = nvm_bin.to_string_lossy();

    let status = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "[Environment]::SetEnvironmentVariable('NVM_BIN', '{}', 'User')",
                nvm_bin_str
            ),
        ])
        .status()
        .map_err(|e| with_context("Failed to set NVM_BIN", e))?;

    if !status.success() {
        return Err(message("Failed to set NVM_BIN environment variable"));
    }

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

    Ok(())
}

/// Establece la variable de entorno NVM_NODE
pub fn set_nvm_node(nvm_node: &Path) -> Result<()> {
    use std::ptr;
    use winapi::um::winuser::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    let nvm_node_str = nvm_node.to_string_lossy();

    let status = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "[Environment]::SetEnvironmentVariable('NVM_NODE', '{}', 'User')",
                nvm_node_str
            ),
        ])
        .status()
        .map_err(|e| with_context("Failed to set NVM_NODE", e))?;

    if !status.success() {
        return Err(message("Failed to set NVM_NODE environment variable"));
    }

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

    Ok(())
}

/// Elimina la variable de entorno NVM_HOME
pub fn remove_nvm_home() -> Result<()> {
    use std::ptr;
    use winapi::um::winuser::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    // Eliminar NVM_HOME
    let status = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "[Environment]::SetEnvironmentVariable('NVM_HOME', $null, 'User')",
        ])
        .status()
        .map_err(|e| with_context("Failed to remove NVM_HOME", e))?;

    if !status.success() {
        return Err(message("Failed to remove NVM_HOME environment variable"));
    }

    // Notificar al sistema del cambio
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

    Ok(())
}

pub fn get_path_instructions(install_dir: &Path) -> String {
    format!(
        r#"Para agregar nvm al PATH permanentemente:
1. Abrir PowerShell como Administrador
2. Ejecutar: $env:PATH += ";{}"
3. O agregar manualmente a las Variables de Entorno del Sistema"#,
        install_dir.display()
    )
}
