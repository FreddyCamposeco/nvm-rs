use anyhow::Result;
use crate::t;

use crate::config::Config;
use crate::core::versions;
use crate::error::message;

pub async fn which(version: Option<String>, config: &Config) -> Result<()> {
    let resolved = if let Some(v) = version {
        let available = crate::core::get_cached_versions(config).await?;
        versions::resolve_version(&v, &available)?
    } else {
        versions::get_current_version(config)
            .ok_or_else(|| message("No version currently active. Run: nvm use <version>"))?
    };

    let version_dir = config.versions_dir().join(&resolved);

    #[cfg(windows)]
    let node_bin = version_dir.join("node.exe");

    #[cfg(not(windows))]
    let node_bin = version_dir.join("bin").join("node");

    if node_bin.exists() {
        println!("{}", node_bin.display());
    } else {
        eprintln!("{}", t!("which_not_installed").replace("{}", &resolved));
        eprintln!("Run: nvm install {}", resolved);
        std::process::exit(1);
    }

    Ok(())
}
