use anyhow::Result;
use crate::t;

use crate::config::Config;
use crate::core;

use crate::utils::{print_check, print_success, print_warning, print_x};

/// Show all Node.js installations found in the system
pub fn show_all_installations() {
    let all_installations = core::detection::find_all_node_installations();
    if !all_installations.is_empty() {
        println!("\n📍 All Node.js Installations ({}):", all_installations.len());
        for (idx, info) in all_installations.iter().enumerate() {
            println!("   {}. {} @ {}", idx + 1, info.version, info.path.display());
            if let Some(npm_ver) = &info.npm_version {
                println!("      npm: {}", npm_ver);
            }
        }
    }
}

/// Show system Node.js detection
pub fn show_system_node() {
    if let Some(system_node) = core::detection::detect_system_node() {
        println!("\n🔍 System Node.js Found:");
        println!("   Version: {}", system_node.version);
        println!("   Path: {}", system_node.path.display());
        if let Some(npm_ver) = system_node.npm_version {
            println!("   npm: {}", npm_ver);
        }
    }
}

/// Run full doctor diagnostics
pub fn run_diagnostics(config: &Config, _fix: bool) -> Result<()> {
    println!("\n{}", t!("doctor_title"));
    println!("{}", "=".repeat(50));

    // Check NVM directory
    if config.nvm_dir.exists() {
        print_check();
        println!(
            "{}: {}",
            t!("doctor_directory", ""),
            config.nvm_dir.display()
        );
    } else {
        print_x();
        println!("NVM directory does not exist");
    }

    // Check installed versions
    let installed = core::get_installed_versions(config)?;
    print_check();
    println!("{}: {}", t!("doctor_installed_versions"), installed.len());

    // Check connectivity (simple test)
    print!("{} ", t!("doctor_connectivity"));
    // Use blocking client to avoid async issues
    let handle = std::thread::spawn(|| {
        reqwest::blocking::get("https://nodejs.org")
    });

    match handle.join() {
        Ok(Ok(_)) => print_success("OK"),
        _ => print_warning("Failed"),
    }

    // Check symlink support
    print!("{} ", t!("doctor_symlink_support"));

    #[cfg(unix)]
    {
        print_success("Supported");
    }

    #[cfg(windows)]
    {
        // Test actual junction/symlink creation on Windows
        let test_dir = config.nvm_dir.join(".symlink-test");
        let test_target = test_dir.join("target");
        let test_link = test_dir.join("link");

        let symlink_works = if let Ok(_) = std::fs::create_dir_all(&test_target) {
            use junction::create as create_junction;
            match create_junction(&test_target, &test_link) {
                Ok(_) => {
                    // Clean up
                    let _ = std::fs::remove_dir(&test_link);
                    let _ = std::fs::remove_dir(&test_target);
                    let _ = std::fs::remove_dir(&test_dir);
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        };

        if symlink_works {
            print_success("Supported");
        } else {
            print_warning("Check required (admin rights may be needed)");
            println!("\n  {} To enable: Run Windows as Administrator", t!("note"));
            println!("  {} Or enable Developer Mode for non-admin symlink creation", t!("note"));
        }
    }

    // Check PATH and environment variables (Unix)
    #[cfg(not(windows))]
    {
        let expected_home = config.nvm_dir.to_string_lossy().to_string();
        let expected_bin = config.nvm_dir.join("bin");
        let expected_node = config.nvm_dir.join("current").join("bin");
        let expected_node_exists = expected_node.exists();

        let env_home = std::env::var("NVM_HOME").ok();
        let env_bin = std::env::var("NVM_BIN").ok();
        let env_node = std::env::var("NVM_NODE").ok();

        let env_ok = env_home.as_deref() == Some(expected_home.as_str())
            && env_bin.as_deref() == Some(expected_bin.to_string_lossy().as_ref())
            && (!expected_node_exists
                || env_node.as_deref() == Some(expected_node.to_string_lossy().as_ref()));

        let path_ok = core::installer::is_in_path(&expected_bin)
            && (!expected_node_exists || core::installer::is_in_path(&expected_node));

        print!("NVM env & PATH ");
        if env_ok && path_ok {
            print_success("OK");
        } else {
            print_warning("Missing or incomplete");

            if fix {
                let mut fixed = true;
                if let Err(e) = core::installer::set_nvm_dir(&config.nvm_dir) {
                    fixed = false;
                    print_warning(&format!("Failed to update env: {}", e));
                }

                if let Err(e) = core::installer::add_to_path(&expected_bin) {
                    fixed = false;
                    print_warning(&format!("Failed to update PATH: {}", e));
                }

                if fixed {
                    print_success("Updated shell configuration");
                    println!("\n  {} Restart your terminal to apply changes", t!("note"));
                }
            }
        }
    }

    println!();
    Ok(())
}
