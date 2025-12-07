# Cross-Compilation Script for nvm-rs
# This script helps build binaries for multiple platforms

Write-Host "Cross-Compilation Setup for nvm-rs" -ForegroundColor Green
Write-Host "==================================" -ForegroundColor Green

# Get the project root directory (parent of scripts directory)
$projectRoot = Split-Path -Parent $PSScriptRoot
Write-Host "Project root: $projectRoot" -ForegroundColor Gray

# Change to project root
Push-Location $projectRoot

try {
    # Check if rustup is available
    if (Get-Command rustup -ErrorAction SilentlyContinue) {
        Write-Host "Rustup found! Installing targets..." -ForegroundColor Yellow

        # Install targets
        rustup target add x86_64-unknown-linux-gnu
        rustup target add aarch64-apple-darwin

        Write-Host "Targets installed successfully!" -ForegroundColor Green

        # Build for Linux
        Write-Host "Building for Linux x86_64..." -ForegroundColor Yellow
        cargo build --release --target x86_64-unknown-linux-gnu
        if ($LASTEXITCODE -ne 0) {
            throw "Failed to build for Linux"
        }

        # Build for macOS ARM
        Write-Host "Building for macOS ARM64..." -ForegroundColor Yellow
        cargo build --release --target aarch64-apple-darwin
        if ($LASTEXITCODE -ne 0) {
            throw "Failed to build for macOS ARM"
        }

        Write-Host "Cross-compilation completed successfully!" -ForegroundColor Green
        Write-Host "Binaries available in target/ directory:" -ForegroundColor Cyan
        Write-Host "  - target/x86_64-unknown-linux-gnu/release/nvm (Linux)" -ForegroundColor White
        Write-Host "  - target/aarch64-apple-darwin/release/nvm (macOS ARM)" -ForegroundColor White
    } else {
        Write-Host "Rustup not found in PATH. Please install rustup or add it to PATH." -ForegroundColor Red
        Write-Host "Download from: https://rustup.rs/" -ForegroundColor Yellow
        Write-Host "Or use alternative compilation methods in CROSS-COMPILE.md" -ForegroundColor Yellow
        exit 1
    }
}
catch {
    Write-Host "Cross-compilation failed. This is common on Windows." -ForegroundColor Red
    Write-Host "Alternative options:" -ForegroundColor Yellow
    Write-Host "1. Use GitHub Actions: Push to trigger automatic builds" -ForegroundColor White
    Write-Host "2. Use Docker: docker run --rm -v ${pwd}:/app -w /app rust:latest cargo build --release" -ForegroundColor White
    Write-Host "3. Use WSL: Install Ubuntu WSL and build natively" -ForegroundColor White
    Write-Host "4. See CROSS-COMPILE.md for detailed instructions" -ForegroundColor White
    exit 1
}
finally {
    # Always return to original location
    Pop-Location
}