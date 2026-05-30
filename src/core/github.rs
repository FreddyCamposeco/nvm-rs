// GitHub API functions for nvm self-update and releases
use crate::error::{message, with_context, Result};
use reqwest;
use serde::Deserialize;
use std::env;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
pub struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

const GITHUB_REPO_OWNER: &str = "FreddyCamposeco";
const GITHUB_REPO_NAME: &str = "nvm-rs";

/// Obtiene la última release disponible en GitHub
pub async fn get_latest_release() -> Result<GithubRelease> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        GITHUB_REPO_OWNER, GITHUB_REPO_NAME
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "nvm-rs-installer")
        .send()
        .await
        .map_err(|e| with_context("Failed to fetch latest release", e))?;

    if !response.status().is_success() {
        return Err(message(format!(
            "Failed to get latest release: {}",
            response.status()
        )));
    }

    let release: GithubRelease = response
        .json()
        .await
        .map_err(|e| with_context("Failed to parse release information", e))?;

    Ok(release)
}

/// Obtiene una release específica por tag
pub async fn get_release_by_tag(tag: &str) -> Result<GithubRelease> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/tags/{}",
        GITHUB_REPO_OWNER, GITHUB_REPO_NAME, tag
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "nvm-rs-installer")
        .send()
        .await
        .map_err(|e| with_context("Failed to fetch release", e))?;

    if !response.status().is_success() {
        return Err(message(format!("Release {} not found", tag)));
    }

    let release: GithubRelease = response
        .json()
        .await
        .map_err(|e| with_context("Failed to parse release information", e))?;

    Ok(release)
}

/// Determina el nombre del asset apropiado para la plataforma actual
pub fn get_platform_asset_name(version: &str, with_self_update: bool) -> String {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    let suffix = if with_self_update { "-self-update" } else { "" };

    match (os, arch) {
        ("windows", "x86_64") => format!("nvm-{}-windows-x64{}.exe", version, suffix),
        ("windows", "x86") => format!("nvm-{}-windows-x86{}.exe", version, suffix),
        ("linux", "x86_64") => format!("nvm-{}-linux-x64{}", version, suffix),
        ("linux", "aarch64") => format!("nvm-{}-linux-arm64{}", version, suffix),
        ("macos", "x86_64") => format!("nvm-{}-macos-x64{}", version, suffix),
        ("macos", "aarch64") => format!("nvm-{}-macos-arm64{}", version, suffix),
        _ => format!("nvm-{}-{}-{}{}", version, os, arch, suffix),
    }
}

/// Verifies the SHA256 integrity of a downloaded asset against the release's checksums file.
/// Looks for a `checksums.txt` or `{asset.name}.sha256` companion asset in the release.
/// Returns `Ok(true)` if verified, `Ok(false)` if no checksum file is present in the release
/// (caller should warn), or `Err` if the checksum mismatches.
pub async fn verify_asset_integrity(
    release: &GithubRelease,
    asset: &GithubAsset,
    downloaded_path: &Path,
) -> Result<bool> {
    use crate::core::download::calculate_checksum;

    let sha256_name = format!("{}.sha256", asset.name);
    let checksum_asset = release
        .assets
        .iter()
        .find(|a| a.name == "checksums.txt" || a.name == sha256_name);

    let Some(checksum_asset) = checksum_asset else {
        return Ok(false);
    };

    let client = reqwest::Client::new();
    let content = client
        .get(&checksum_asset.browser_download_url)
        .header("User-Agent", "nvm-rs-installer")
        .send()
        .await
        .map_err(|e| with_context("Failed to download checksum file", e))?
        .text()
        .await
        .map_err(|e| with_context("Failed to read checksum file", e))?;

    let expected = content
        .lines()
        .find_map(|line| {
            let mut parts = line.split_whitespace();
            let hash = parts.next()?;
            let name = parts.next().unwrap_or("");
            if name == asset.name || (checksum_asset.name == sha256_name && !hash.is_empty()) {
                Some(hash.to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| message(format!("Checksum not found for {}", asset.name)))?;

    let actual = calculate_checksum(downloaded_path)?;
    if actual.to_lowercase() != expected.to_lowercase() {
        return Err(message(format!(
            "Asset integrity check FAILED for {}\nExpected: {}\nActual:   {}",
            asset.name, expected, actual
        )));
    }

    Ok(true)
}

/// Descarga un asset desde GitHub
pub async fn download_asset(asset: &GithubAsset, dest_path: &Path) -> Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::fs;
    use std::io::Write;
    use futures_util::StreamExt;

    let client = reqwest::Client::new();
    let response = client
        .get(&asset.browser_download_url)
        .header("User-Agent", "nvm-rs-installer")
        .send()
        .await
        .map_err(|e| with_context("Failed to download asset", e))?;

    if !response.status().is_success() {
        return Err(message(format!(
            "Download failed with status: {}",
            response.status()
        )));
    }

    // Configurar barra de progreso
    let total_size = asset.size;
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Descargar con progreso
    let mut file = fs::File::create(dest_path)
        .map_err(|e| with_context("Failed to create destination file", e))?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| with_context("Error while downloading", e))?;
        file.write_all(&chunk)
            .map_err(|e| with_context("Failed to write to file", e))?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}
