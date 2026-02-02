use crate::error::{with_context, Result};
use std::path::Path;

#[cfg(windows)]
use junction::create as create_junction;

#[cfg(unix)]
use std::os::unix::fs as unix_fs;

/// Crea o actualiza un symlink/junction apuntando a la versión de Node.js
///
/// En Windows usa junctions (similar a symlinks pero no requiere permisos admin)
/// En Unix usa symlinks estándar
pub fn create_or_update_symlink(target: &Path, link: &Path) -> Result<()> {
    // Si el link ya existe, eliminarlo primero
    if link.exists() || link.symlink_metadata().is_ok() {
        remove_symlink(link)?;
    }

    // Crear el directorio padre si no existe
    if let Some(parent) = link.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| with_context("Failed to create parent directory for symlink", e))?;
    }

    // Crear el symlink según la plataforma
    #[cfg(windows)]
    {
        create_junction(target, link).map_err(|e| {
            with_context(
                &format!(
                    "Failed to create junction from {} to {}",
                    link.display(),
                    target.display()
                ),
                e,
            )
        })?;
    }

    #[cfg(unix)]
    {
        unix_fs::symlink(target, link).map_err(|e| {
            with_context(
                &format!(
                    "Failed to create symlink from {} to {}",
                    link.display(),
                    target.display()
                ),
                e,
            )
        })?;
    }

    Ok(())
}

/// Persiste la versión actual en un archivo .nvm-version para recuperación confiable
pub fn persist_current_version(link: &Path, version: &str) -> Result<()> {
    // Crear directorio 'current' si no existe
    if let Some(parent) = link.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| with_context("Failed to create current directory", e))?;
    }

    let version_file = link.join(".nvm-version");
    std::fs::write(&version_file, version).map_err(|e| {
        with_context(
            &format!("Failed to persist version to {}", version_file.display()),
            e,
        )
    })?;

    Ok(())
}

/// Lee la versión persistida desde archivo .nvm-version
#[allow(dead_code)] // Will be used in Phase 2 (version detection improvements)
pub fn read_persisted_version(link: &Path) -> Result<Option<String>> {
    let version_file = link.join(".nvm-version");

    if !version_file.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&version_file).map_err(|e| {
        with_context(
            &format!("Failed to read persisted version from {}", version_file.display()),
            e,
        )
    })?;

    let version = content.trim().to_string();
    if version.is_empty() {
        Ok(None)
    } else {
        Ok(Some(version))
    }
}

/// Elimina un symlink/junction existente
pub fn remove_symlink(link: &Path) -> Result<()> {
    if !link.exists() && link.symlink_metadata().is_err() {
        // El link no existe, no hay nada que hacer
        return Ok(());
    }

    #[cfg(windows)]
    {
        // En Windows, usar remove_dir para junctions
        if link.is_dir() {
            std::fs::remove_dir(link).map_err(|e| {
                with_context(&format!("Failed to remove junction: {}", link.display()), e)
            })?;
        } else {
            std::fs::remove_file(link).map_err(|e| {
                with_context(&format!("Failed to remove symlink: {}", link.display()), e)
            })?;
        }
    }

    #[cfg(unix)]
    {
        // En Unix, remove_file funciona para symlinks
        std::fs::remove_file(link).map_err(|e| {
            with_context(&format!("Failed to remove symlink: {}", link.display()), e)
        })?;
    }

    Ok(())
}

/// Verifica si un symlink existe y es válido (apunta a un directorio existente)
pub fn is_valid_symlink(link: &Path) -> bool {
    if let Ok(metadata) = link.symlink_metadata() {
        if metadata.is_symlink() || metadata.is_dir() {
            // Verificar que el target existe
            return link.exists();
        }
    }
    false
}

/// Lee el target de un symlink
#[allow(dead_code)] // Will be used in future phases for verification
pub fn read_symlink_target(link: &Path) -> Result<std::path::PathBuf> {
    #[cfg(windows)]
    {
        // En Windows, leer el junction manualmente
        std::fs::read_link(link)
            .or_else(|_| {
                // Fallback: si read_link falla, intentar canonicalize
                link.canonicalize()
            })
            .map_err(|e| {
                with_context(&format!("Failed to read symlink target: {}", link.display()), e)
            })
    }

    #[cfg(unix)]
    {
        std::fs::read_link(link).map_err(|e| {
            with_context(&format!("Failed to read symlink target: {}", link.display()), e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_create_and_remove_symlink() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("target");
        let link = temp.path().join("link");

        // Crear directorio target
        fs::create_dir(&target).unwrap();
        fs::write(target.join("test.txt"), b"test").unwrap();

        // Crear symlink
        let result = create_or_update_symlink(&target, &link);
        assert!(result.is_ok(), "Should create symlink: {:?}", result.err());

        #[cfg(windows)]
        assert!(link.exists(), "Link should exist on Windows");

        #[cfg(unix)]
        {
            assert!(link.symlink_metadata().is_ok(), "Link metadata should be readable");
            assert!(link.exists(), "Link should exist");
        }

        // Verificar que es válido
        assert!(is_valid_symlink(&link), "Symlink should be valid");

        // Verificar que podemos leer a través del symlink
        let content = fs::read_to_string(link.join("test.txt")).unwrap();
        assert_eq!(content, "test");

        // Eliminar symlink
        let result = remove_symlink(&link);
        assert!(result.is_ok(), "Should remove symlink");
        assert!(!link.exists(), "Link should not exist after removal");
    }

    #[test]
    fn test_update_existing_symlink() {
        let temp = TempDir::new().unwrap();
        let target1 = temp.path().join("target1");
        let target2 = temp.path().join("target2");
        let link = temp.path().join("link");

        // Crear directorios target
        fs::create_dir(&target1).unwrap();
        fs::create_dir(&target2).unwrap();
        fs::write(target1.join("version.txt"), b"1").unwrap();
        fs::write(target2.join("version.txt"), b"2").unwrap();

        // Crear primer symlink
        create_or_update_symlink(&target1, &link).unwrap();
        let content = fs::read_to_string(link.join("version.txt")).unwrap();
        assert_eq!(content, "1");

        // Actualizar a target2
        create_or_update_symlink(&target2, &link).unwrap();
        let content = fs::read_to_string(link.join("version.txt")).unwrap();
        assert_eq!(content, "2");
    }

    #[test]
    fn test_is_valid_symlink() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("target");
        let link = temp.path().join("link");

        // Sin crear, no es válido
        assert!(!is_valid_symlink(&link));

        // Crear target y symlink
        fs::create_dir(&target).unwrap();
        create_or_update_symlink(&target, &link).unwrap();

        // Ahora es válido
        assert!(is_valid_symlink(&link));

        // Eliminar target (romper el symlink)
        fs::remove_dir(&target).unwrap();

        // Ya no es válido (broken symlink)
        #[cfg(unix)]
        assert!(!is_valid_symlink(&link));
    }
}
