use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

/// Creates a configured HTTP client with appropriate headers and timeout
#[allow(dead_code)] // Will be used in Phase 3 (download)
pub fn create_client() -> Result<Client> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("nvm-rs/0.1.0")
        .build()
        .context("Failed to create HTTP client")?;
    
    Ok(client)
}

/// Downloads a file from the given URL and saves it to the destination path
/// Shows a progress bar during download
#[allow(dead_code)] // Will be used in Phase 3 (install)
pub async fn download_file(url: &str, dest: &Path) -> Result<()> {
    let client = create_client()?;
    
    // Make initial request to get content length
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send HTTP request")?;
    
    let total_size = response.content_length().unwrap_or(0);
    
    // Create progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    
    // Download content
    let content = response
        .bytes()
        .await
        .context("Failed to download file")?;
    
    pb.set_position(content.len() as u64);
    pb.finish_with_message("Download complete");
    
    // Write to file
    let mut file = File::create(dest)
        .context(format!("Failed to create file: {}", dest.display()))?;
    
    file.write_all(&content)
        .context("Failed to write downloaded content to file")?;
    
    Ok(())
}

/// Downloads JSON data from a URL with retry logic
#[allow(dead_code)] // Will be used in Phase 3-4 (install, version checking)
pub async fn download_json(url: &str, max_retries: u32) -> Result<String> {
    let client = create_client()?;
    let mut last_error = None;
    
    for attempt in 0..=max_retries {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return response
                        .text()
                        .await
                        .context("Failed to read response body");
                } else {
                    last_error = Some(anyhow::anyhow!(
                        "HTTP error: {}",
                        response.status()
                    ));
                }
            }
            Err(e) => {
                last_error = Some(anyhow::anyhow!("Request failed: {}", e));
            }
        }
        
        if attempt < max_retries {
            // Wait before retrying (exponential backoff)
            let wait_time = Duration::from_secs(2_u64.pow(attempt));
            tokio::time::sleep(wait_time).await;
        }
    }
    
    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Download failed after {} retries", max_retries)))
}

/// Checks if a URL is accessible (HEAD request)
#[allow(dead_code)] // Used in tests and will be used in Phase 5 (verification)
pub async fn check_url(url: &str) -> bool {
    let client = match create_client() {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    match client.head(url).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_url_success() {
        let result = check_url("https://nodejs.org/dist/index.json").await;
        assert!(result, "nodejs.org should be accessible");
    }

    #[tokio::test]
    async fn test_check_url_failure() {
        let result = check_url("https://invalid-domain-that-does-not-exist-12345.com").await;
        assert!(!result, "Invalid domain should not be accessible");
    }

    #[tokio::test]
    async fn test_download_json() {
        let result = download_json("https://nodejs.org/dist/index.json", 2).await;
        assert!(result.is_ok(), "Should download Node.js version index");
        
        let json = result.unwrap();
        assert!(json.contains("version"), "Response should contain version data");
    }
}
