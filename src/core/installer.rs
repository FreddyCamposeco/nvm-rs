// Re-export download functions from download module for self-update functionality
pub use super::download::calculate_checksum;

use crate::error::{message, with_context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Obtiene la ruta del ejecutable actual
pub fn get_current_executable() -> Result<PathBuf> {
    env::current_exe().map_err(|e| with_context("Failed to get current executable path", e))
}

/// Obtiene el directorio de instalación recomendado
pub fn get_install_dir() -> Result<PathBuf> {
    #[cfg(windows)]
    {
        // En Windows, instalar en %USERPROFILE%\.nvm\bin
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".nvm").join("bin");
            return Ok(path);
        }
        Ok(PathBuf::from("C:\\nvm\\bin"))
    }

    #[cfg(not(windows))]
    {
        // En Unix, instalar en $NVM_HOME/bin para homologar con scripts
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".nvm").join("bin");
            return Ok(path);
        }
        Ok(PathBuf::from("/usr/local/bin"))
    }
}

/// Instala el binario descargado en el sistema
pub fn install_binary(source: &Path, install_dir: &Path) -> Result<PathBuf> {
    // Crear directorio de instalación si no existe
    fs::create_dir_all(install_dir)
        .map_err(|e| with_context("Failed to create installation directory", e))?;

    // Determinar nombre del ejecutable
    #[cfg(windows)]
    let exe_name = "nvm.exe";
    #[cfg(not(windows))]
    let exe_name = "nvm";

    let dest_path = install_dir.join(exe_name);

    // Si el archivo destino existe, hacer backup
    if dest_path.exists() {
        let backup_path = dest_path.with_extension("exe.bak");
        fs::rename(&dest_path, &backup_path)
            .map_err(|e| with_context("Failed to backup existing binary", e))?;
    }

    // Copiar el nuevo binario
    fs::copy(source, &dest_path)
        .map_err(|e| with_context("Failed to copy binary to installation directory", e))?;

    // En Unix, hacer ejecutable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest_path, perms)?;
    }

    Ok(dest_path)
}

/// Desinstala nvm del sistema
#[allow(dead_code)]
pub fn uninstall_binary(install_dir: Option<&Path>) -> Result<()> {
    let install_dir = if let Some(dir) = install_dir {
        dir.to_path_buf()
    } else {
        get_install_dir()?
    };

    #[cfg(windows)]
    let exe_name = "nvm.exe";
    #[cfg(not(windows))]
    let exe_name = "nvm";

    let exe_path = install_dir.join(exe_name);

    if !exe_path.exists() {
        return Err(message(format!(
            "nvm binary not found at {}",
            exe_path.display()
        )));
    }

    // Eliminar el binario
    fs::remove_file(&exe_path)
        .map_err(|e| with_context("Failed to remove nvm binary", e))?;

    // Eliminar backup si existe
    #[cfg(windows)]
    let backup_path = exe_path.with_extension("exe.bak");
    #[cfg(not(windows))]
    let backup_path = install_dir.join("nvm.bak");

    if backup_path.exists() {
        let _ = fs::remove_file(&backup_path);
    }

    Ok(())
}

/// Verifica si nvm está en el PATH
pub fn is_in_path(install_dir: &Path) -> bool {
    crate::platform::path::is_in_path(install_dir)
}

/// Genera instrucciones para agregar al PATH
pub fn get_path_instructions(_install_dir: &Path) -> String {
    #[cfg(windows)]
    {
        crate::platform::windows::env::get_path_instructions(_install_dir)
    }

    #[cfg(not(windows))]
    {
        crate::platform::unix::env::get_path_instructions()
    }
}

/// Agrega el directorio al PATH del usuario (permanente)
#[cfg(windows)]
pub fn add_to_path(install_dir: &Path) -> Result<()> {
    crate::platform::windows::env::add_to_path(install_dir)
}

/// Elimina el directorio del PATH del usuario
#[cfg(windows)]
pub fn remove_from_path(install_dir: &Path) -> Result<()> {
    crate::platform::windows::env::remove_from_path(install_dir)
}

/// Establece la variable de entorno NVM_HOME
#[cfg(windows)]
pub fn set_nvm_home(nvm_dir: &Path) -> Result<()> {
    crate::platform::windows::env::set_nvm_home(nvm_dir)
}

/// Establece la variable de entorno NVM_BIN
#[cfg(windows)]
pub fn set_nvm_bin(nvm_bin: &Path) -> Result<()> {
    crate::platform::windows::env::set_nvm_bin(nvm_bin)
}

/// Establece la variable de entorno NVM_NODE
#[cfg(windows)]
pub fn set_nvm_node(nvm_node: &Path) -> Result<()> {
    crate::platform::windows::env::set_nvm_node(nvm_node)
}

/// Elimina la variable de entorno NVM_HOME
#[cfg(windows)]
#[allow(dead_code)]
pub fn remove_nvm_home() -> Result<()> {
    crate::platform::windows::env::remove_nvm_home()
}

/// Limpieza completa de instalación - elimina TODOS los rastros de nvm
/// Incluye: binario, variables de entorno, PATH, directorios de datos
#[cfg(windows)]
pub fn full_uninstall_cleanup(install_dir: Option<&Path>, data_dir: Option<&Path>, purge: bool, remove_config: bool) -> Result<()> {
    crate::platform::windows::uninstall::full_uninstall_cleanup(install_dir, data_dir, purge, remove_config)?;
    Ok(())
}

// Unix versions (stub implementations for non-Windows)
#[cfg(not(windows))]
pub fn full_uninstall_cleanup(install_dir: Option<&Path>, data_dir: Option<&Path>, purge: bool, remove_config: bool) -> Result<()> {
    crate::platform::unix::uninstall::full_uninstall_cleanup(install_dir, data_dir, purge, remove_config)?;
    Ok(())
}

// Unix versions (stub implementations for non-Windows)
// Reserved for future phases (automated shell configuration)
#[cfg(not(windows))]
#[allow(dead_code)]
pub fn add_to_path(_install_dir: &Path) -> Result<()> {
    let install_dir = _install_dir;
    let nvm_dir = install_dir
        .parent()
        .unwrap_or(install_dir)
        .to_path_buf();

    let (config_path, shell) = crate::platform::unix::shell::detect_shell_config()?;
    let block = crate::platform::unix::shell::build_shell_block(&nvm_dir, shell);
    crate::platform::unix::shell::ensure_shell_block(&config_path, &block)?;
    Ok(())
}

#[cfg(not(windows))]
#[allow(dead_code)]
pub fn remove_from_path(_install_dir: &Path) -> Result<()> {
    let (config_path, _) = crate::platform::unix::shell::detect_shell_config()?;
    crate::platform::unix::shell::remove_shell_block(&config_path)?;
    Ok(())
}

#[cfg(not(windows))]
#[allow(dead_code)]
pub fn set_nvm_dir(_nvm_dir: &Path) -> Result<()> {
    let nvm_dir = _nvm_dir;
    let (config_path, shell) = crate::platform::unix::shell::detect_shell_config()?;
    let block = crate::platform::unix::shell::build_shell_block(nvm_dir, shell);
    crate::platform::unix::shell::ensure_shell_block(&config_path, &block)?;
    Ok(())
}

#[cfg(not(windows))]
#[allow(dead_code)]
pub fn remove_nvm_dir() -> Result<()> {
    let (config_path, _) = crate::platform::unix::shell::detect_shell_config()?;
    crate::platform::unix::shell::remove_shell_block(&config_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::github::get_platform_asset_name;

    #[test]
    fn test_get_platform_asset_name() {
        let name = get_platform_asset_name("v0.1.0", false);
        assert!(name.starts_with("nvm-v0.1.0"));
    }

    #[test]
    fn test_get_install_dir() {
        let dir = get_install_dir();
        assert!(dir.is_ok());
    }
}
