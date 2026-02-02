use crate::error::{with_context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Estructura para almacenar aliases de versiones de Node.js
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aliases {
    /// Mapa de nombre de alias -> versión
    #[serde(flatten)]
    pub aliases: HashMap<String, String>,
}

impl Aliases {
    /// Crea una instancia vacía de Aliases
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }

    /// Obtiene la ruta del archivo de aliases
    pub fn get_aliases_file_path() -> Result<PathBuf> {
        let config = crate::config::Config::new()?;
        Ok(config.nvm_dir.join("aliases.json"))
    }

    /// Carga los aliases desde el archivo
    pub fn load() -> Result<Self> {
        let path = Self::get_aliases_file_path()?;

        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&path).map_err(|e| {
            with_context(
                &format!("Failed to read aliases file: {}", path.display()),
                e,
            )
        })?;

        if content.trim().is_empty() {
            return Ok(Self::new());
        }

        let aliases: Aliases = serde_json::from_str(&content).map_err(|e| {
            with_context(
                &format!("Failed to parse aliases file: {}", path.display()),
                e,
            )
        })?;

        Ok(aliases)
    }

    /// Guarda los aliases en el archivo
    pub fn save(&self) -> Result<()> {
        let path = Self::get_aliases_file_path()?;

        // Crear directorio si no existe
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                with_context(
                    &format!("Failed to create directory: {}", parent.display()),
                    e,
                )
            })?;
        }

        let content = serde_json::to_string_pretty(&self)
            .map_err(|e| with_context("Failed to serialize aliases", e))?;

        fs::write(&path, content).map_err(|e| {
            with_context(
                &format!("Failed to write aliases file: {}", path.display()),
                e,
            )
        })?;

        Ok(())
    }

    /// Añade o actualiza un alias
    pub fn set(&mut self, name: String, version: String) {
        self.aliases.insert(name, version);
    }

    /// Elimina un alias
    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.aliases.remove(name)
    }

    /// Obtiene la versión asociada a un alias
    pub fn get(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }

    /// Verifica si existe un alias
    pub fn contains(&self, name: &str) -> bool {
        self.aliases.contains_key(name)
    }

    /// Obtiene todos los aliases ordenados por nombre
    pub fn list(&self) -> Vec<(&String, &String)> {
        let mut aliases: Vec<_> = self.aliases.iter().collect();
        aliases.sort_by(|a, b| a.0.cmp(b.0));
        aliases
    }

    /// Verifica si un nombre de alias es válido
    pub fn is_valid_alias_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        // No permitir nombres que empiecen con 'v' seguido de un dígito (para evitar confusión con versiones)
        if name.starts_with('v') && name.len() > 1 {
            if let Some(second_char) = name.chars().nth(1) {
                if second_char.is_ascii_digit() {
                    return false;
                }
            }
        }

        // Solo permitir letras, números, guiones y guiones bajos
        name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '/')
    }

    /// Resuelve un alias a una versión concreta
    /// Si el input ya es una versión, la retorna sin cambios
    /// Si es un alias, retorna la versión asociada
    pub fn resolve(&self, name_or_version: &str) -> Option<String> {
        // Si empieza con 'v' y tiene dígitos, probablemente es una versión
        if name_or_version.starts_with('v') {
            if let Some(second_char) = name_or_version.chars().nth(1) {
                if second_char.is_ascii_digit() {
                    return Some(name_or_version.to_string());
                }
            }
        }

        // Intentar resolver como alias
        if let Some(version) = self.get(name_or_version) {
            return Some(version.clone());
        }

        // Si no es alias ni versión, retornar None
        None
    }
}

impl Default for Aliases {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_aliases() {
        let aliases = Aliases::new();
        assert!(aliases.aliases.is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let mut aliases = Aliases::new();
        aliases.set("default".to_string(), "v20.10.0".to_string());
        
        assert_eq!(aliases.get("default"), Some(&"v20.10.0".to_string()));
        assert_eq!(aliases.get("nonexistent"), None);
    }

    #[test]
    fn test_remove() {
        let mut aliases = Aliases::new();
        aliases.set("default".to_string(), "v20.10.0".to_string());
        
        let removed = aliases.remove("default");
        assert_eq!(removed, Some("v20.10.0".to_string()));
        assert!(!aliases.contains("default"));
    }

    #[test]
    fn test_contains() {
        let mut aliases = Aliases::new();
        aliases.set("default".to_string(), "v20.10.0".to_string());
        
        assert!(aliases.contains("default"));
        assert!(!aliases.contains("nonexistent"));
    }

    #[test]
    fn test_list() {
        let mut aliases = Aliases::new();
        aliases.set("default".to_string(), "v20.10.0".to_string());
        aliases.set("stable".to_string(), "v22.0.0".to_string());
        aliases.set("lts".to_string(), "v20.10.0".to_string());
        
        let list = aliases.list();
        assert_eq!(list.len(), 3);
        
        // Verificar que están ordenados
        assert_eq!(list[0].0, "default");
        assert_eq!(list[1].0, "lts");
        assert_eq!(list[2].0, "stable");
    }

    #[test]
    fn test_valid_alias_names() {
        assert!(Aliases::is_valid_alias_name("default"));
        assert!(Aliases::is_valid_alias_name("stable"));
        assert!(Aliases::is_valid_alias_name("lts"));
        assert!(Aliases::is_valid_alias_name("my-alias"));
        assert!(Aliases::is_valid_alias_name("my_alias"));
        assert!(Aliases::is_valid_alias_name("lts/iron"));
        assert!(Aliases::is_valid_alias_name("vtest")); // 'v' seguido de letra es válido
        
        // Inválidos
        assert!(!Aliases::is_valid_alias_name("")); // vacío
        assert!(!Aliases::is_valid_alias_name("v20")); // parece versión
        assert!(!Aliases::is_valid_alias_name("v20.10.0")); // parece versión
    }

    #[test]
    fn test_resolve() {
        let mut aliases = Aliases::new();
        aliases.set("default".to_string(), "v20.10.0".to_string());
        aliases.set("stable".to_string(), "v22.0.0".to_string());
        
        // Resolver alias
        assert_eq!(aliases.resolve("default"), Some("v20.10.0".to_string()));
        assert_eq!(aliases.resolve("stable"), Some("v22.0.0".to_string()));
        
        // Versiones directas
        assert_eq!(aliases.resolve("v20.10.0"), Some("v20.10.0".to_string()));
        assert_eq!(aliases.resolve("v22.0.0"), Some("v22.0.0".to_string()));
        
        // No existe
        assert_eq!(aliases.resolve("nonexistent"), None);
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut aliases = Aliases::new();
        aliases.set("default".to_string(), "v20.10.0".to_string());
        aliases.set("stable".to_string(), "v22.0.0".to_string());
        
        let json = serde_json::to_string(&aliases).unwrap();
        let deserialized: Aliases = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.get("default"), Some(&"v20.10.0".to_string()));
        assert_eq!(deserialized.get("stable"), Some(&"v22.0.0".to_string()));
    }
}
