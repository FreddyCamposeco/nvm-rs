use anyhow::Result;
use std::path::PathBuf;

// Environment variables homologation
pub const NVM_HOME_VAR: &str = "NVM_HOME";
#[allow(dead_code)] // Reserved for future phases
pub const NVM_BIN_VAR: &str = "NVM_BIN";
#[allow(dead_code)] // Reserved for future phases
pub const NVM_NODE_VAR: &str = "NVM_NODE";
pub const NODE_MIRROR_VAR: &str = "NODE_MIRROR";

#[derive(Debug, Clone)]
pub struct Config {
    pub nvm_dir: PathBuf,
    pub node_mirror: String,
    pub arch: String,
    pub cache_duration_minutes: u64,
    pub installed_cache_duration_minutes: u64,
}

impl Config {
    pub fn new() -> Result<Self> {
        let nvm_dir = Self::get_nvm_dir()?;

        // Ensure NVM directory exists
        if !nvm_dir.exists() {
            std::fs::create_dir_all(&nvm_dir)?;
        }

        let arch = Self::detect_arch();

        Ok(Config {
            nvm_dir,
            node_mirror: std::env::var(NODE_MIRROR_VAR)
                .unwrap_or_else(|_| "https://nodejs.org/dist".to_string()),
            arch,
            cache_duration_minutes: 24 * 60,  // 24 hours
            installed_cache_duration_minutes: 5,
        })
    }

    fn get_nvm_dir() -> Result<PathBuf> {
        // Check NVM_HOME environment variable first
        if let Ok(nvm_home) = std::env::var(NVM_HOME_VAR) {
            return Ok(PathBuf::from(nvm_home));
        }

        // Use platform-specific default
        let home_dir = home::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

        Ok(home_dir.join(".nvm"))
    }

    fn detect_arch() -> String {
        #[cfg(target_arch = "x86_64")]
        return "x64".to_string();

        #[cfg(target_arch = "aarch64")]
        return "arm64".to_string();

        #[cfg(target_arch = "x86")]
        return "x86".to_string();

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "x86")))]
        return "unknown".to_string();
    }

    pub fn versions_dir(&self) -> PathBuf {
        self.nvm_dir.join("versions")
    }

    /// Returns the NVM_NODE directory: $NVM_HOME/current/bin (active Node binaries)
    /// This is homologated across platforms:
    /// - Windows: %NVM_HOME%\current\bin
    /// - Unix:    $NVM_HOME/current/bin
    pub fn nvm_node_dir(&self) -> PathBuf {
        self.nvm_dir.join("current").join("bin")
    }

    pub fn current_dir(&self) -> PathBuf {
        self.nvm_node_dir()
    }

    pub fn cache_file(&self) -> PathBuf {
        self.nvm_dir.join(".version_cache.json")
    }

    pub fn installed_cache_file(&self) -> PathBuf {
        self.nvm_dir.join(".installed_cache.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("Failed to create default config")
    }
}
