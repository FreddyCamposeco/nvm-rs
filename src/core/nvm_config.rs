use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::{with_context, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NvmConfig {
    pub locale: Option<String>,
    pub default_version: Option<String>,
}

impl NvmConfig {
    pub fn load(config_path: &Path) -> Self {
        if !config_path.exists() {
            return Self::default();
        }
        std::fs::read_to_string(config_path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, config_path: &Path) -> Result<()> {
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| with_context("Failed to create config directory", e))?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| with_context("Failed to serialize nvm config", e))?;
        std::fs::write(config_path, content)
            .map_err(|e| with_context("Failed to write nvm config", e))
    }
}
