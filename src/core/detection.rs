// Module for detecting system Node.js installations
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;

use colored::Colorize;

/// Información sobre una instalación de Node.js detectada
#[derive(Debug, Clone)]
pub struct SystemNodeInfo {
    pub path: PathBuf,
    pub version: String,
    pub npm_version: Option<String>,
    #[allow(dead_code)]
    pub source: DetectionSource,
}

/// Fuente de detección
#[derive(Debug, Clone, PartialEq)]
pub enum DetectionSource {
    /// Encontrado en PATH
    PathEnvironment,
    /// Instalación del sistema (Program Files, /usr/local, etc.)
    SystemInstallation,
    /// NVM (no debería detectarse aquí, pero lo marcamos)
    #[allow(dead_code)]
    NvmManaged,
}

impl std::fmt::Display for DetectionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectionSource::PathEnvironment => write!(f, "PATH"),
            DetectionSource::SystemInstallation => write!(f, "System"),
            DetectionSource::NvmManaged => write!(f, "NVM"),
        }
    }
}

/// Detectar Node.js en el sistema (primero en PATH)
pub fn detect_system_node() -> Option<SystemNodeInfo> {
    // Primero intentar en PATH
    if let Some(info) = find_node_in_path() {
        return Some(info);
    }

    // Luego buscar en ubicaciones comunes del sistema
    find_node_in_system_locations()
}

/// Buscar Node.js en PATH
fn find_node_in_path() -> Option<SystemNodeInfo> {
    let cmd = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };

    if let Ok(output) = Command::new(cmd)
        .arg("node")
        .output()
    {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                let path = PathBuf::from(&path_str);

                // Verificar que no sea de NVM
                if !is_nvm_managed(&path) {
                    if let Some(version) = get_node_version(&path) {
                        let npm_version = get_npm_version(&path);
                        return Some(SystemNodeInfo {
                            path,
                            version,
                            npm_version,
                            source: DetectionSource::PathEnvironment,
                        });
                    }
                }
            }
        }
    }

    None
}

/// Buscar Node.js en ubicaciones comunes del sistema
fn find_node_in_system_locations() -> Option<SystemNodeInfo> {
    let paths = get_common_node_paths();

    for path in paths {
        if path.exists() {
            let node_path = if cfg!(target_os = "windows") {
                path.join("node.exe")
            } else {
                path.join("node")
            };

            if node_path.exists() && !is_nvm_managed(&node_path) {
                if let Some(version) = get_node_version(&node_path) {
                    let npm_version = get_npm_version(&node_path);
                    return Some(SystemNodeInfo {
                        path: node_path,
                        version,
                        npm_version,
                        source: DetectionSource::SystemInstallation,
                    });
                }
            }
        }
    }

    None
}

/// Obtener rutas comunes donde se instala Node.js
fn get_common_node_paths() -> Vec<PathBuf> {
    let mut paths = vec![];

    #[cfg(target_os = "windows")]
    {
        paths.push(PathBuf::from("C:\\Program Files\\nodejs"));
        paths.push(PathBuf::from("C:\\Program Files (x86)\\nodejs"));
        if let Ok(userprofile) = env::var("USERPROFILE") {
            paths.push(PathBuf::from(format!(
                "{}\\AppData\\Local\\Programs\\nodejs",
                userprofile
            )));
        }
    }

    #[cfg(target_os = "macos")]
    {
        paths.push(PathBuf::from("/usr/local/bin"));
        paths.push(PathBuf::from("/opt/homebrew/bin"));
        if let Ok(home) = env::var("HOME") {
            paths.push(PathBuf::from(format!("{}/.local/bin", home)));
        }
    }

    #[cfg(target_os = "linux")]
    {
        paths.push(PathBuf::from("/usr/local/bin"));
        paths.push(PathBuf::from("/usr/bin"));
        if let Ok(home) = env::var("HOME") {
            paths.push(PathBuf::from(format!("{}/.local/bin", home)));
        }
    }

    paths
}

/// Obtener versión de Node.js
fn get_node_version(node_path: &PathBuf) -> Option<String> {
    if let Ok(output) = Command::new(node_path)
        .arg("--version")
        .output()
    {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !version.is_empty() {
                return Some(version);
            }
        }
    }

    None
}

/// Obtener versión de npm
fn get_npm_version(node_path: &Path) -> Option<String> {
    // Assumir que npm está en el mismo directorio que node
    let node_dir = node_path.parent()?;
    let npm_path = if cfg!(target_os = "windows") {
        node_dir.join("npm.cmd")
    } else {
        node_dir.join("npm")
    };

    if npm_path.exists() {
        if let Ok(output) = Command::new(&npm_path)
            .arg("--version")
            .output()
        {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !version.is_empty() {
                    return Some(version);
                }
            }
        }
    }

    None
}

/// Verificar si la ruta es manejada por NVM
fn is_nvm_managed(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    path_str.contains(".nvm") || path_str.contains("nvm")
}

/// Encontrar todas las instalaciones de Node.js en el sistema
pub fn find_all_node_installations() -> Vec<SystemNodeInfo> {
    let mut installations = vec![];

    // Primero agregar la de PATH si existe
    if let Some(info) = detect_system_node() {
        installations.push(info);
    }

    // Luego buscar en ubicaciones comunes
    let paths = get_common_node_paths();
    for path in paths {
        if path.exists() {
            let node_path = if cfg!(target_os = "windows") {
                path.join("node.exe")
            } else {
                path.join("node")
            };

            if node_path.exists()
                && !is_nvm_managed(&node_path)
                && !installations.iter().any(|i| i.path == node_path)
            {
                if let Some(version) = get_node_version(&node_path) {
                    let npm_version = get_npm_version(&node_path);
                    installations.push(SystemNodeInfo {
                        path: node_path,
                        version,
                        npm_version,
                        source: DetectionSource::SystemInstallation,
                    });
                }
            }
        }
    }

    installations
}

/// Mostrar información del Node.js del sistema
#[allow(dead_code)]
pub fn display_system_node(info: &SystemNodeInfo) {
    println!(
        "\n{} System Node.js Installation Detected:\n",
        "✓".green()
    );
    println!("  {} {}", "Path:".cyan(), info.path.display());
    println!("  {} {}", "Version:".cyan(), info.version);

    if let Some(npm_ver) = &info.npm_version {
        println!("  {} {}", "npm:".cyan(), npm_ver);
    }

    println!("  {} {}\n", "Source:".cyan(), info.source);
}

/// Mostrar todas las instalaciones encontradas
#[allow(dead_code)]
pub fn display_all_installations(installations: &[SystemNodeInfo]) {
    if installations.is_empty() {
        println!("\n{} No system Node.js installations found\n", "⚠".yellow());
        return;
    }

    println!(
        "\n{} Found {} Node.js installation(s):\n",
        "✓".green(),
        installations.len()
    );

    for (i, info) in installations.iter().enumerate() {
        println!("  {}. {}", i + 1, format!("Node.js {}", info.version).cyan());
        println!("     {} {}", "Path:".yellow(), info.path.display());
        if let Some(npm_ver) = &info.npm_version {
            println!("     {} {}", "npm:".yellow(), npm_ver);
        }
        println!("     {} {}", "Source:".yellow(), info.source);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nvm_managed() {
        let nvm_path = PathBuf::from("/home/user/.nvm/versions/node/v20.0.0/bin/node");
        assert!(is_nvm_managed(&nvm_path));

        let system_path = PathBuf::from("/usr/local/bin/node");
        assert!(!is_nvm_managed(&system_path));
    }

    #[test]
    fn test_common_paths_windows() {
        #[cfg(target_os = "windows")]
        {
            let paths = get_common_node_paths();
            assert!(paths.iter().any(|p| p.to_string_lossy().contains("Program Files")));
        }
    }

    #[test]
    fn test_common_paths_unix() {
        #[cfg(target_os = "linux")]
        {
            let paths = get_common_node_paths();
            assert!(paths.iter().any(|p| p.to_string_lossy().contains("/usr")));
        }
    }
}
