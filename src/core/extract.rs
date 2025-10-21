use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
use zip::ZipArchive;

#[cfg(not(target_os = "windows"))]
use flate2::read::GzDecoder;
#[cfg(not(target_os = "windows"))]
use tar::Archive;

/// Extrae un archivo comprimido al directorio de destino
pub fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<PathBuf> {
    println!("Extracting to: {}", dest_dir.display());
    
    #[cfg(target_os = "windows")]
    {
        extract_zip(archive_path, dest_dir)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        extract_tar_gz(archive_path, dest_dir)
    }
}

#[cfg(target_os = "windows")]
fn extract_zip(archive_path: &Path, dest_dir: &Path) -> Result<PathBuf> {
    let file = fs::File::open(archive_path)
        .context(format!("Failed to open archive: {}", archive_path.display()))?;
    
    let mut archive = ZipArchive::new(file)
        .context("Failed to read ZIP archive")?;
    
    // Crear directorio de destino
    fs::create_dir_all(dest_dir).context("Failed to create destination directory")?;
    
    let mut extracted_root = None;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("Failed to read ZIP entry")?;
        let outpath = match file.enclosed_name() {
            Some(path) => dest_dir.join(path),
            None => continue,
        };
        
        // Guardar el primer directorio como raíz extraída
        if extracted_root.is_none() {
            if let Some(first_component) = file.name().split('/').next() {
                extracted_root = Some(dest_dir.join(first_component));
            }
        }
        
        if file.name().ends_with('/') {
            // Es un directorio
            fs::create_dir_all(&outpath).context("Failed to create directory")?;
        } else {
            // Es un archivo
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).context("Failed to create parent directory")?;
                }
            }
            
            let mut outfile = fs::File::create(&outpath)
                .context(format!("Failed to create file: {}", outpath.display()))?;
            
            std::io::copy(&mut file, &mut outfile)
                .context("Failed to extract file")?;
        }
    }
    
    println!("Extraction complete");
    
    extracted_root.ok_or_else(|| anyhow::anyhow!("No files extracted from archive"))
}

#[cfg(not(target_os = "windows"))]
fn extract_tar_gz(archive_path: &Path, dest_dir: &Path) -> Result<PathBuf> {
    let file = fs::File::open(archive_path)
        .context(format!("Failed to open archive: {}", archive_path.display()))?;
    
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    
    // Crear directorio de destino
    fs::create_dir_all(dest_dir).context("Failed to create destination directory")?;
    
    let mut extracted_root = None;
    
    // Extraer archivos
    for entry in archive.entries().context("Failed to read tar entries")? {
        let mut entry = entry.context("Failed to read tar entry")?;
        let path = entry.path().context("Failed to get entry path")?;
        
        // Guardar el primer directorio como raíz extraída
        if extracted_root.is_none() {
            if let Some(first_component) = path.components().next() {
                extracted_root = Some(dest_dir.join(first_component.as_os_str()));
            }
        }
        
        let outpath = dest_dir.join(&*path);
        
        // Extraer archivo/directorio
        entry.unpack(&outpath)
            .context(format!("Failed to extract: {}", path.display()))?;
        
        // En Unix, preservar permisos ejecutables
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mode) = entry.header().mode() {
                if mode & 0o111 != 0 {
                    // Tiene permisos de ejecución
                    let mut perms = fs::metadata(&outpath)?.permissions();
                    perms.set_mode(mode);
                    fs::set_permissions(&outpath, perms)?;
                }
            }
        }
    }
    
    println!("Extraction complete");
    
    extracted_root.ok_or_else(|| anyhow::anyhow!("No files extracted from archive"))
}

/// Mueve el contenido extraído a la ubicación final
/// Node.js típicamente extrae a un directorio como "node-v20.10.0-win-x64"
/// pero queremos moverlo a "v20.10.0"
pub fn move_extracted_files(extracted_path: &Path, target_path: &Path) -> Result<()> {
    println!("Moving files from {} to {}", extracted_path.display(), target_path.display());
    
    if !extracted_path.exists() {
        anyhow::bail!("Extracted path does not exist: {}", extracted_path.display());
    }
    
    // Si el target ya existe, eliminarlo
    if target_path.exists() {
        fs::remove_dir_all(target_path)
            .context(format!("Failed to remove existing target: {}", target_path.display()))?;
    }
    
    // Crear el directorio padre del target
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create target parent directory")?;
    }
    
    // Mover/renombrar el directorio extraído
    fs::rename(extracted_path, target_path)
        .context(format!(
            "Failed to move from {} to {}",
            extracted_path.display(),
            target_path.display()
        ))?;
    
    println!("Files moved successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_extract_archive_structure() {
        // Este test solo verifica que las funciones existen y son llamables
        // Tests reales requerirían archivos de prueba
        let temp_dir = std::env::temp_dir().join("nvm_test_extract");
        let _ = fs::create_dir_all(&temp_dir);
        
        // Limpiar
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_move_extracted_files() {
        let temp_dir = std::env::temp_dir().join("nvm_test_move");
        let _ = fs::create_dir_all(&temp_dir);
        
        let source = temp_dir.join("source");
        let target = temp_dir.join("target");
        
        // Crear directorio fuente con un archivo
        fs::create_dir_all(&source).unwrap();
        let test_file = source.join("test.txt");
        fs::File::create(&test_file).unwrap()
            .write_all(b"test").unwrap();
        
        // Mover
        let result = move_extracted_files(&source, &target);
        assert!(result.is_ok(), "Move should succeed");
        assert!(target.exists(), "Target should exist");
        assert!(!source.exists(), "Source should not exist");
        assert!(target.join("test.txt").exists(), "File should be moved");
        
        // Limpiar
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
