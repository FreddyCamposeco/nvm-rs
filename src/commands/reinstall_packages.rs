use anyhow::{anyhow, Context, Result};
use crate::t;
use crate::config::Config;
use crate::core::{get_cached_versions, versions};
use std::process::Command;

pub async fn reinstall_packages(from_version: &str, config: &Config) -> Result<()> {
    let available = get_cached_versions(config).await?;
    let from_resolved = versions::resolve_version(from_version, &available)?;

    let current_version = versions::get_current_version(config)
        .ok_or_else(|| anyhow!("{}", t!("no_active_version")))?;

    if from_resolved == current_version {
        return Err(anyhow!("{}", t!("reinstall_same_version").replace("{version}", &from_resolved)));
    }

    let from_dir = config.versions_dir().join(&from_resolved);
    if !from_dir.exists() {
        return Err(anyhow!(
            "{}",
            t!("run_version_not_installed").replace("{}", &from_resolved)
        ));
    }

    #[cfg(windows)]
    let from_bin = from_dir.clone();
    #[cfg(not(windows))]
    let from_bin = from_dir.join("bin");

    let npm_path = from_bin.join(if cfg!(windows) { "npm.cmd" } else { "npm" });
    if !npm_path.exists() {
        return Err(anyhow!("npm not found in version {}", from_resolved));
    }

    println!("{}", t!("reinstall_listing").replace("{version}", &from_resolved));

    // Get globally installed packages from source version
    let output = Command::new(&npm_path)
        .args(["list", "--global", "--json", "--depth=0"])
        .output()
        .with_context(|| format!("Failed to run npm from {}", from_resolved))?;

    let json_str = String::from_utf8_lossy(&output.stdout);
    let packages = parse_npm_global_list(&json_str, &from_resolved)?;

    if packages.is_empty() {
        println!("{}", t!("reinstall_no_packages"));
        return Ok(());
    }

    println!("{}", t!("reinstall_found").replace("{count}", &packages.len().to_string()));
    for (name, ver) in &packages {
        println!("  {}@{}", name, ver);
    }
    println!();

    // Get npm from current (target) version
    let current_dir = config.versions_dir().join(&current_version);
    #[cfg(windows)]
    let current_bin = current_dir.clone();
    #[cfg(not(windows))]
    let current_bin = current_dir.join("bin");

    let current_npm = current_bin.join(if cfg!(windows) { "npm.cmd" } else { "npm" });
    if !current_npm.exists() {
        return Err(anyhow!("npm not found in current version {}", current_version));
    }

    println!("{}", t!("reinstall_installing").replace("{version}", &current_version));

    let mut failed = Vec::new();
    for (name, ver) in &packages {
        let pkg_spec = format!("{}@{}", name, ver);
        print!("  {} ... ", pkg_spec);
        use std::io::Write;
        let _ = std::io::stdout().flush();

        let result = Command::new(&current_npm)
            .args(["install", "--global", &pkg_spec])
            .output();

        match result {
            Ok(out) if out.status.success() => println!("ok"),
            Ok(out) => {
                println!("failed");
                let stderr = String::from_utf8_lossy(&out.stderr);
                let stdout = String::from_utf8_lossy(&out.stdout);
                if !stderr.trim().is_empty() {
                    eprintln!("{}", stderr.trim());
                } else if !stdout.trim().is_empty() {
                    eprintln!("{}", stdout.trim());
                }
                failed.push(pkg_spec);
            }
            Err(e) => {
                println!("failed ({})", e);
                failed.push(pkg_spec);
            }
        }
    }

    if failed.is_empty() {
        println!("\n{}", t!("reinstall_complete").replace("{count}", &packages.len().to_string()));
    } else {
        println!(
            "\n{}",
            t!("reinstall_partial")
                .replace("{ok}", &(packages.len() - failed.len()).to_string())
                .replace("{failed}", &failed.len().to_string())
        );
        for pkg in &failed {
            println!("  - {}", pkg);
        }
    }

    Ok(())
}

fn parse_npm_global_list(json: &str, skip_version: &str) -> Result<Vec<(String, String)>> {
    // npm list --json output: { "dependencies": { "pkg": { "version": "x.y.z" } } }
    let trimmed = json.trim();
    if trimmed.is_empty() {
        return Ok(vec![]);
    }

    let mut packages = Vec::new();

    // Minimal JSON parsing without extra deps — look for "dependencies" object
    // Use serde_json if available, otherwise simple regex-free extraction
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
        if let Some(deps) = val.get("dependencies").and_then(|d| d.as_object()) {
            for (name, info) in deps {
                if name == "npm" {
                    continue;
                }
                let version = info
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("latest");
                packages.push((name.clone(), version.to_string()));
            }
        }
    } else {
        return Err(anyhow!(
            "Failed to parse npm list output for version {}",
            skip_version
        ));
    }

    Ok(packages)
}
