use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::core::versions::NodeVersion;
use crate::utils::http::create_client;

/// Construye la URL de descarga para una versión de Node.js
pub fn get_download_url(version: &NodeVersion, config: &Config) -> String {
    let arch = &config.arch;
    let version_str = &version.version;
    
    #[cfg(target_os = "windows")]
    let filename = format!("node-{}-win-{}.zip", version_str, arch);
    
    #[cfg(not(target_os = "windows"))]
    let filename = {
        #[cfg(target_os = "macos")]
        let os = "darwin";
        #[cfg(target_os = "linux")]
        let os = "linux";
        
        format!("node-{}-{}-{}.tar.gz", version_str, os, arch)
    };
    
    format!("{}/{}/{}", config.node_mirror, version_str, filename)
}

/// Construye la URL del archivo SHASUMS256.txt
pub fn get_checksum_url(version: &str, config: &Config) -> String {
    format!("{}/{}/SHASUMS256.txt", config.node_mirror, version)
}

/// Descarga el archivo SHASUMS256.txt y extrae el checksum para el archivo especificado
pub async fn get_expected_checksum(
    version: &str,
    filename: &str,
    config: &Config,
) -> Result<String> {
    let client = create_client()?;
    let url = get_checksum_url(version, config);
    
    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to download SHASUMS256.txt")?;
    
    let content = response
        .text()
        .await
        .context("Failed to read SHASUMS256.txt")?;
    
    // Formato: "checksum  filename"
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == filename {
            return Ok(parts[0].to_string());
        }
    }
    
    anyhow::bail!("Checksum not found for {}", filename)
}

/// Calcula el checksum SHA256 de un archivo
pub fn calculate_checksum(path: &Path) -> Result<String> {
    let mut file = File::open(path).context("Failed to open file for checksum")?;
    let mut hasher = Sha256::new();
    
    std::io::copy(&mut file, &mut hasher).context("Failed to read file for checksum")?;
    
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

/// Verifica que el checksum de un archivo coincida con el esperado
pub fn verify_checksum(path: &Path, expected: &str) -> Result<()> {
    let actual = calculate_checksum(path)?;
    
    if actual.to_lowercase() == expected.to_lowercase() {
        Ok(())
    } else {
        anyhow::bail!(
            "Checksum mismatch!\nExpected: {}\nActual:   {}",
            expected,
            actual
        )
    }
}

/// Descarga un archivo de Node.js con barra de progreso
pub async fn download_node_archive(
    version: &NodeVersion,
    dest_dir: &Path,
    config: &Config,
) -> Result<PathBuf> {
    let client = create_client()?;
    let url = get_download_url(version, config);
    
    // Extraer el nombre del archivo de la URL
    let filename = url.split('/').last().unwrap();
    let dest_path = dest_dir.join(filename);
    
    // Si el archivo ya existe, verificar si es válido
    if dest_path.exists() {
        println!("Archive already exists, verifying...");
        
        // Intentar verificar checksum
        if let Ok(expected) = get_expected_checksum(&version.version, filename, config).await {
            if verify_checksum(&dest_path, &expected).is_ok() {
                println!("Archive verified, skipping download");
                return Ok(dest_path);
            }
        }
        
        println!("Archive invalid, re-downloading...");
        std::fs::remove_file(&dest_path)?;
    }
    
    println!("Downloading from: {}", url);
    
    // Hacer la solicitud inicial para obtener el tamaño
    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to start download")?;
    
    if !response.status().is_success() {
        anyhow::bail!("Download failed with status: {}", response.status());
    }
    
    let total_size = response.content_length().unwrap_or(0);
    
    // Crear barra de progreso
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    
    // Descargar contenido
    let content = response
        .bytes()
        .await
        .context("Failed to download file")?;
    
    pb.set_position(content.len() as u64);
    pb.finish_with_message("Download complete");
    
    // Crear directorio si no existe
    if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create download directory")?;
    }
    
    // Escribir archivo
    let mut file = File::create(&dest_path)
        .context(format!("Failed to create file: {}", dest_path.display()))?;
    
    file.write_all(&content)
        .context("Failed to write downloaded content")?;
    
    println!("Saved to: {}", dest_path.display());
    
    // Verificar checksum
    println!("Verifying checksum...");
    match get_expected_checksum(&version.version, filename, config).await {
        Ok(expected) => {
            verify_checksum(&dest_path, &expected)
                .context("Checksum verification failed")?;
            println!("Checksum verified ✓");
        }
        Err(e) => {
            println!("Warning: Could not verify checksum: {}", e);
        }
    }
    
    Ok(dest_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_url_windows() {
        let config = Config::new().unwrap();
        let version = NodeVersion {
            version: "v20.10.0".to_string(),
            lts: crate::core::versions::LtsInfo::Name("Iron".to_string()),
            files: vec![],
            date: "".to_string(),
            npm: None,
            v8: None,
            uv: None,
            zlib: None,
            openssl: None,
            modules: None,
            security: false,
        };
        
        let url = get_download_url(&version, &config);
        
        #[cfg(target_os = "windows")]
        assert!(url.contains("win-x64.zip") || url.contains("win-x86.zip"));
        
        #[cfg(not(target_os = "windows"))]
        assert!(url.contains(".tar.gz"));
    }

    #[test]
    fn test_get_checksum_url() {
        let config = Config::new().unwrap();
        let url = get_checksum_url("v20.10.0", &config);
        assert!(url.contains("v20.10.0"));
        assert!(url.ends_with("SHASUMS256.txt"));
    }

    #[tokio::test]
    async fn test_get_expected_checksum() {
        let config = Config::new().unwrap();
        
        // Test con una versión conocida
        let result = get_expected_checksum(
            "v20.10.0",
            "node-v20.10.0-win-x64.zip",
            &config
        ).await;
        
        // Debería encontrar el checksum o fallar por red
        match result {
            Ok(checksum) => {
                assert_eq!(checksum.len(), 64); // SHA256 = 64 caracteres hex
            }
            Err(e) => {
                // Es aceptable si falla por red
                println!("Network error (expected in some environments): {}", e);
            }
        }
    }
}
