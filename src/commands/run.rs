use anyhow::{anyhow, Context, Result};
use crate::t;

use crate::config::Config;
use crate::core::{get_cached_versions, versions};

pub async fn run_with_version(version: &str, command: &[String], config: &Config) -> Result<()> {
    if command.is_empty() {
        return Err(anyhow!("{}", t!("run_no_command")));
    }

    let available = get_cached_versions(config).await?;
    let resolved = versions::resolve_version(version, &available)?;

    let version_dir = config.versions_dir().join(&resolved);

    #[cfg(windows)]
    let bin_dir = version_dir.clone();

    #[cfg(not(windows))]
    let bin_dir = version_dir.join("bin");

    if !bin_dir.exists() {
        return Err(anyhow!(
            "{}",
            t!("run_version_not_installed").replace("{}", &resolved)
        ));
    }

    let current_path = std::env::var("PATH").unwrap_or_default();
    let sep = if cfg!(windows) { ";" } else { ":" };
    let new_path = format!("{}{}{}", bin_dir.display(), sep, current_path);

    let (cmd, args) = command.split_first().unwrap();

    let status = std::process::Command::new(cmd)
        .args(args)
        .env("PATH", &new_path)
        .status()
        .with_context(|| format!("Failed to run '{}'", cmd))?;

    std::process::exit(status.code().unwrap_or(1));
}
