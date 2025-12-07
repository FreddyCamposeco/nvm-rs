use std::env;
use std::path::PathBuf;

pub struct Config {
    pub nvm_dir: PathBuf,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let home_dir = env::var("USERPROFILE")
            .or_else(|_| env::var("HOME"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));

        let nvm_dir = env::var("NVM_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home_dir.join(".nvm"));

        // Ensure the directory exists
        std::fs::create_dir_all(&nvm_dir)?;

        Ok(Config { nvm_dir })
    }
}
