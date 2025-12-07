use crate::config::Config;
use crate::t;
use crate::utils::*;
use std::fs;

pub fn list_versions(config: &Config) -> anyhow::Result<()> {
    let versions_dir = config.nvm_dir.join("versions");

    if !versions_dir.exists() {
        print_warning(&t!("no_versions_installed"));
        return Ok(());
    }

    let entries = fs::read_dir(&versions_dir)?;
    let mut installed_versions = Vec::new();

    for entry in entries {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with('v') {
                installed_versions.push(name.to_string());
            }
        }
    }

    if installed_versions.is_empty() {
        print_warning(&t!("no_versions_installed"));
        return Ok(());
    }

    // Get current version
    let current_version = get_current_version(config)?;

    // Get system version
    let system_version = get_system_version();

    println!();

    // Show system version if exists
    if let Some(sys_ver) = &system_version {
        let normalized_sys_ver = if sys_ver.starts_with('v') {
            sys_ver.clone()
        } else {
            format!("v{}", sys_ver)
        };
        let is_current =
            Some(normalized_sys_ver.as_str()) == current_version.as_ref().map(|s| s.as_str());

        print_version_line(
            "system",
            &t!("system_label"),
            &normalized_sys_ver,
            true,
            is_current,
            false,
        );
    }

    // For now, show latest as the highest installed version
    if let Some(latest_installed) = installed_versions.iter().max() {
        let is_current =
            Some(latest_installed.as_str()) == current_version.as_ref().map(|s| s.as_str());
        let is_installed = installed_versions.contains(latest_installed);

        print_version_line(
            "latest",
            &t!("latest_label"),
            latest_installed,
            is_installed,
            is_current,
            false,
        );
    }

    // Show some common LTS versions (simplified - in real implementation this would come from remote data)
    let lts_versions = vec![("iron", "v20.10.0"), ("jod", "v18.19.0")];

    for (lts_name, lts_version) in lts_versions {
        let is_installed = installed_versions.contains(&lts_version.to_string());
        let is_current = Some(lts_version) == current_version.as_ref().map(|s| s.as_str());

        let label = t!("lts_label", lts_name);
        print_version_line("lts", &label, lts_version, is_installed, is_current, false);
    }

    // Show installed versions
    println!("{}", t!("installed_versions"));
    for version in &installed_versions {
        let is_current = Some(version.as_str()) == current_version.as_ref().map(|s| s.as_str());

        print_version_line("installed", "", version, true, is_current, false);
    }

    println!();

    Ok(())
}

fn print_version_line(
    version_type: &str,
    label: &str,
    version: &str,
    is_installed: bool,
    is_current: bool,
    has_update: bool,
) {
    // Determine indicator based on type
    let indicator = match version_type {
        "system" => {
            if is_current {
                "▶"
            } else {
                " "
            }
        }
        "global" => "→",
        "latest" => {
            if is_current {
                "▶"
            } else {
                " "
            }
        }
        "lts" => {
            if is_current {
                "▶"
            } else {
                " "
            }
        }
        "nvmrc" => {
            if is_current {
                "▶"
            } else {
                "ϟ"
            }
        }
        _ => {
            if is_current {
                "▶"
            } else {
                " "
            }
        }
    };

    // Determine colors
    let (label_color, version_color) = match version_type {
        "system" => (COLOR_YELLOW, COLOR_YELLOW),
        "global" => (
            COLOR_GRAY,
            if is_installed {
                COLOR_CYAN
            } else {
                COLOR_DARK_GRAY
            },
        ),
        "latest" => (
            COLOR_GRAY,
            if is_installed {
                COLOR_CYAN
            } else {
                COLOR_DARK_GRAY
            },
        ),
        "lts" => (
            COLOR_YELLOW,
            if is_installed {
                COLOR_CYAN
            } else {
                COLOR_DARK_GRAY
            },
        ),
        "nvmrc" => (
            COLOR_MAGENTA,
            if is_installed {
                COLOR_MAGENTA
            } else {
                COLOR_DARK_GRAY
            },
        ),
        _ => (
            COLOR_GRAY,
            if is_installed {
                COLOR_CYAN
            } else {
                COLOR_DARK_GRAY
            },
        ),
    };

    // Print indicator
    print_colored_no_newline(
        indicator,
        if is_current { COLOR_GREEN } else { COLOR_RESET },
    );
    print!(" ");

    // Print label (right-aligned in 14-character field)
    print_colored_no_newline(&format!("{:>14}", label), label_color);
    print!(" ");

    // Print version
    print_colored_no_newline(version, version_color);

    // Calculate space to align check mark at column 27
    let spaces_needed = 27 - 17 - version.len();
    for _ in 0..spaces_needed.max(0) {
        print!(" ");
    }

    // Print installation indicator
    if is_installed {
        if has_update {
            print_colored("✓▲", COLOR_YELLOW);
        } else {
            print_colored("✓", COLOR_GREEN);
        }
    } else {
        print!(" ");
    }

    println!();
}

pub fn show_current_version(config: &Config) -> anyhow::Result<()> {
    match get_current_version(config)? {
        Some(version) => println!("{}", version),
        None => print_warning(&t!("no_current_version")),
    }
    Ok(())
}

#[cfg(windows)]
fn get_current_version_from_path(current_path: &std::path::Path) -> Option<String> {
    // Try to read version from .nvm-version file first
    if let Ok(version_content) = fs::read_to_string(current_path.join(".nvm-version")) {
        return Some(version_content.trim().to_string());
    }

    // Fallback: Check if the current directory contains the dummy node.exe file
    if current_path.join("node.exe").exists() {
        // This fallback should rarely be needed now that we store .nvm-version
        None
    } else {
        None
    }
}

pub fn use_version(config: &Config, version: &str) -> anyhow::Result<()> {
    let resolved_version = resolve_version(version)?;
    let version_path = config.nvm_dir.join("versions").join(&resolved_version);
    let current_path = config.nvm_dir.join("current");

    if !version_path.exists() {
        return Err(anyhow::anyhow!(
            "{}",
            t!("version_not_installed", &resolved_version)
        ));
    }

    // Remove existing current if it exists
    if current_path.exists() {
        #[cfg(unix)]
        fs::remove_file(&current_path)?;
        #[cfg(windows)]
        {
            // On Windows, try to remove file, but if it's a symlink it might fail
            let _ = fs::remove_file(&current_path);
            let _ = fs::remove_dir_all(&current_path);
        }
    }

    // Create symlink or copy on Windows
    #[cfg(unix)]
    std::os::unix::fs::symlink(&version_path, &current_path)?;

    #[cfg(windows)]
    {
        // On Windows, try symlink first, fallback to copy if it fails
        match std::os::windows::fs::symlink_dir(&version_path, &current_path) {
            Ok(_) => {}
            Err(_) => {
                // Fallback: create a junction or copy
                print_info(&t!("symlink_fallback"));
                copy_dir_recursive(&version_path, &current_path)?;
            }
        }
    }

    // Store the current version in a file for reliable detection
    fs::write(
        current_path.join(".nvm-version"),
        resolved_version.as_bytes(),
    )?;

    print_success(&t!("now_using_node", &resolved_version));

    Ok(())
}

pub fn list_remote_versions(_config: &Config) -> anyhow::Result<()> {
    // For MVP, show some hardcoded versions
    let versions = vec![
        "v20.10.0", "v20.9.0", "v20.8.1", "v18.18.2", "v18.17.1", "v16.20.2",
    ];

    println!("{}", t!("remote_versions"));
    for version in versions {
        println!("  {}", version);
    }

    Ok(())
}

pub fn resolve_version(version: &str) -> anyhow::Result<String> {
    if version.starts_with('v') {
        Ok(version.to_string())
    } else {
        Ok(format!("v{}", version))
    }
}

#[cfg(windows)]
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> anyhow::Result<()> {
    if dst.exists() {
        fs::remove_dir_all(dst)?;
    }
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

pub fn get_current_version(config: &Config) -> anyhow::Result<Option<String>> {
    // Check if current symlink exists
    let current_path = config.nvm_dir.join("current");

    if current_path.exists() {
        // Try to read version from .nvm-version file first (works on all platforms)
        if let Ok(version_content) = fs::read_to_string(current_path.join(".nvm-version")) {
            return Ok(Some(version_content.trim().to_string()));
        }

        // On Windows, it might be a copied directory instead of symlink
        #[cfg(windows)]
        {
            if current_path.is_dir() {
                // Try to find the version by checking if it's a known version directory
                if let Some(version) = get_current_version_from_path(&current_path) {
                    return Ok(Some(version));
                }
            }
        }

        // Try to read as symlink
        if let Ok(target) = fs::read_link(&current_path) {
            if let Some(version) = target.file_name().and_then(|n| n.to_str()) {
                return Ok(Some(version.to_string()));
            }
        }
    }

    Ok(None)
}

pub fn get_system_version() -> Option<String> {
    // Try to find system Node.js installation
    if let Ok(output) = std::process::Command::new("node").arg("--version").output() {
        if output.status.success() {
            if let Ok(version) = String::from_utf8(output.stdout) {
                return Some(version.trim().to_string());
            }
        }
    }
    None
}
