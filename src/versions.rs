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
    let mut versions = Vec::new();

    for entry in entries {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with('v') {
                versions.push(name.to_string());
            }
        }
    }

    if versions.is_empty() {
        print_warning(&t!("no_versions_installed"));
    } else {
        println!("{}", t!("installed_versions"));
        for version in versions {
            println!("  {}", version);
        }
    }

    Ok(())
}

pub fn show_current_version(config: &Config) -> anyhow::Result<()> {
    // Check if current symlink exists
    let current_path = config.nvm_dir.join("current");

    if current_path.exists() {
        // Try to read version from .nvm-version file first (works on all platforms)
        if let Ok(version_content) = fs::read_to_string(current_path.join(".nvm-version")) {
            println!("{}", version_content.trim());
            return Ok(());
        }

        // On Windows, it might be a copied directory instead of symlink
        #[cfg(windows)]
        {
            if current_path.is_dir() {
                // Try to find the version by checking if it's a known version directory
                if let Some(version) = get_current_version_from_path(&current_path) {
                    println!("{}", version);
                    return Ok(());
                }
            }
        }

        // Try to read as symlink
        if let Ok(target) = fs::read_link(&current_path) {
            if let Some(version) = target.file_name().and_then(|n| n.to_str()) {
                println!("{}", version);
                return Ok(());
            }
        }
    }

    print_warning(&t!("no_current_version"));
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
