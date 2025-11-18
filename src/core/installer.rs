use anyhow::{Context, Result};
use reqwest;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Sha256, Digest};
use std::io::Write;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
pub struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

const GITHUB_REPO_OWNER: &str = "FreddyCamposeco";
const GITHUB_REPO_NAME: &str = "nvm-rs";

/// Obtiene la última release disponible en GitHub
pub async fn get_latest_release() -> Result<GithubRelease> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        GITHUB_REPO_OWNER, GITHUB_REPO_NAME
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "nvm-rs-installer")
        .send()
        .await
        .context("Failed to fetch latest release")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to get latest release: {}", response.status());
    }

    let release: GithubRelease = response.json().await
        .context("Failed to parse release information")?;

    Ok(release)
}

/// Obtiene una release específica por tag
pub async fn get_release_by_tag(tag: &str) -> Result<GithubRelease> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/tags/{}",
        GITHUB_REPO_OWNER, GITHUB_REPO_NAME, tag
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "nvm-rs-installer")
        .send()
        .await
        .context("Failed to fetch release")?;

    if !response.status().is_success() {
        anyhow::bail!("Release {} not found", tag);
    }

    let release: GithubRelease = response.json().await
        .context("Failed to parse release information")?;

    Ok(release)
}

/// Determina el nombre del asset apropiado para la plataforma actual
pub fn get_platform_asset_name(version: &str, with_self_update: bool) -> String {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    let suffix = if with_self_update {
        "-self-update"
    } else {
        ""
    };

    match (os, arch) {
        ("windows", "x86_64") => format!("nvm-{}-windows-x64{}.exe", version, suffix),
        ("windows", "x86") => format!("nvm-{}-windows-x86{}.exe", version, suffix),
        ("linux", "x86_64") => format!("nvm-{}-linux-x64{}", version, suffix),
        ("linux", "aarch64") => format!("nvm-{}-linux-arm64{}", version, suffix),
        ("macos", "x86_64") => format!("nvm-{}-macos-x64{}", version, suffix),
        ("macos", "aarch64") => format!("nvm-{}-macos-arm64{}", version, suffix),
        _ => format!("nvm-{}-{}-{}{}", version, os, arch, suffix),
    }
}

/// Descarga un asset desde GitHub
pub async fn download_asset(asset: &GithubAsset, dest_path: &Path) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .get(&asset.browser_download_url)
        .header("User-Agent", "nvm-rs-installer")
        .send()
        .await
        .context("Failed to download asset")?;

    if !response.status().is_success() {
        anyhow::bail!("Download failed with status: {}", response.status());
    }

    // Configurar barra de progreso
    let total_size = asset.size;
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Descargar con progreso
    let mut file = fs::File::create(dest_path)
        .context("Failed to create destination file")?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("Error while downloading")?;
        file.write_all(&chunk).context("Failed to write to file")?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}

/// Verifica el checksum SHA256 de un archivo
pub async fn verify_checksum(file_path: &Path, expected_checksum: Option<&str>) -> Result<String> {
    let mut file = fs::File::open(file_path)
        .context("Failed to open file for checksum verification")?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    let checksum = format!("{:x}", hash);

    if let Some(expected) = expected_checksum {
        if checksum.to_lowercase() != expected.to_lowercase() {
            anyhow::bail!(
                "Checksum verification failed!\nExpected: {}\nGot: {}",
                expected,
                checksum
            );
        }
    }

    Ok(checksum)
}

/// Obtiene la ruta del ejecutable actual
pub fn get_current_executable() -> Result<PathBuf> {
    env::current_exe().context("Failed to get current executable path")
}

/// Obtiene el directorio de instalación recomendado
pub fn get_install_dir() -> Result<PathBuf> {
    #[cfg(windows)]
    {
        // En Windows, instalar en %LOCALAPPDATA%\Programs\nvm o C:\Program Files\nvm
        if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
            let path = PathBuf::from(local_app_data).join("Programs").join("nvm");
            return Ok(path);
        }
        Ok(PathBuf::from("C:\\Program Files\\nvm"))
    }

    #[cfg(not(windows))]
    {
        // En Unix, instalar en ~/.local/bin o /usr/local/bin
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".local").join("bin");
            return Ok(path);
        }
        Ok(PathBuf::from("/usr/local/bin"))
    }
}

/// Instala el binario descargado en el sistema
pub fn install_binary(source: &Path, install_dir: &Path) -> Result<PathBuf> {
    // Crear directorio de instalación si no existe
    fs::create_dir_all(install_dir)
        .context("Failed to create installation directory")?;

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
            .context("Failed to backup existing binary")?;
    }

    // Copiar el nuevo binario
    fs::copy(source, &dest_path)
        .context("Failed to copy binary to installation directory")?;

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
        anyhow::bail!("nvm binary not found at {}", exe_path.display());
    }

    // Eliminar el binario
    fs::remove_file(&exe_path)
        .context("Failed to remove nvm binary")?;

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
    if let Ok(path_var) = env::var("PATH") {
        let install_dir_str = install_dir.to_string_lossy();
        return path_var
            .split(if cfg!(windows) { ';' } else { ':' })
            .any(|p| p == install_dir_str);
    }
    false
}

/// Genera instrucciones para agregar al PATH
pub fn get_path_instructions(install_dir: &Path) -> String {
    #[cfg(windows)]
    {
        format!(
            r#"Para agregar nvm al PATH permanentemente:
1. Abrir PowerShell como Administrador
2. Ejecutar: $env:PATH += ";{}"
3. O agregar manualmente a las Variables de Entorno del Sistema"#,
            install_dir.display()
        )
    }

    #[cfg(not(windows))]
    {
        let shell_config = if PathBuf::from(env::var("HOME").unwrap_or_default())
            .join(".zshrc")
            .exists()
        {
            "~/.zshrc"
        } else {
            "~/.bashrc"
        };

        format!(
            r#"Para agregar nvm al PATH permanentemente:
1. Agregar al final de {}: export PATH="{}:$PATH"
2. Recargar la configuración: source {}"#,
            shell_config,
            install_dir.display(),
            shell_config
        )
    }
}

/// Agrega el directorio al PATH del usuario (permanente)
#[cfg(windows)]
pub fn add_to_path(install_dir: &Path) -> Result<()> {
    use winapi::um::winuser::{SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};
    use std::ptr;
    
    // Obtener PATH actual del usuario
    let current_path = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "[Environment]::GetEnvironmentVariable('Path', 'User')"
        ])
        .output()
        .context("Failed to get current PATH")?;
    
    let current_path_str = String::from_utf8_lossy(&current_path.stdout).trim().to_string();
    let install_dir_str = install_dir.to_string_lossy();
    
    // Verificar si ya está en el PATH
    if current_path_str.split(';').any(|p| p.trim() == install_dir_str.as_ref()) {
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
            &format!("[Environment]::SetEnvironmentVariable('Path', '{}', 'User')", new_path)
        ])
        .status()
        .context("Failed to set PATH")?;
    
    if !status.success() {
        anyhow::bail!("Failed to update PATH environment variable");
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
#[cfg(windows)]
pub fn remove_from_path(install_dir: &Path) -> Result<()> {
    use winapi::um::winuser::{SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};
    use std::ptr;
    
    // Obtener PATH actual del usuario
    let current_path = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "[Environment]::GetEnvironmentVariable('Path', 'User')"
        ])
        .output()
        .context("Failed to get current PATH")?;
    
    let current_path_str = String::from_utf8_lossy(&current_path.stdout).trim().to_string();
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
            &format!("[Environment]::SetEnvironmentVariable('Path', '{}', 'User')", new_path)
        ])
        .status()
        .context("Failed to set PATH")?;
    
    if !status.success() {
        anyhow::bail!("Failed to update PATH environment variable");
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

/// Establece la variable de entorno NVM_DIR
#[cfg(windows)]
pub fn set_nvm_dir(nvm_dir: &Path) -> Result<()> {
    use winapi::um::winuser::{SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};
    use std::ptr;
    
    let nvm_dir_str = nvm_dir.to_string_lossy();
    
    // Establecer NVM_DIR
    let status = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            &format!("[Environment]::SetEnvironmentVariable('NVM_DIR', '{}', 'User')", nvm_dir_str)
        ])
        .status()
        .context("Failed to set NVM_DIR")?;
    
    if !status.success() {
        anyhow::bail!("Failed to set NVM_DIR environment variable");
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

/// Elimina la variable de entorno NVM_DIR
#[cfg(windows)]
pub fn remove_nvm_dir() -> Result<()> {
    use winapi::um::winuser::{SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};
    use std::ptr;
    
    // Eliminar NVM_DIR
    let status = std::process::Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "[Environment]::SetEnvironmentVariable('NVM_DIR', $null, 'User')"
        ])
        .status()
        .context("Failed to remove NVM_DIR")?;
    
    if !status.success() {
        anyhow::bail!("Failed to remove NVM_DIR environment variable");
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

// Unix versions (stub implementations for non-Windows)
#[cfg(not(windows))]
pub fn add_to_path(_install_dir: &Path) -> Result<()> {
    // En Unix, esto se hace a través de shell config files
    Ok(())
}

#[cfg(not(windows))]
pub fn remove_from_path(_install_dir: &Path) -> Result<()> {
    // En Unix, esto se hace a través de shell config files
    Ok(())
}

#[cfg(not(windows))]
pub fn set_nvm_dir(_nvm_dir: &Path) -> Result<()> {
    // En Unix, esto se hace a través de shell config files
    Ok(())
}

#[cfg(not(windows))]
pub fn remove_nvm_dir() -> Result<()> {
    // En Unix, esto se hace a través de shell config files
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
