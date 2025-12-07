# Cross-Compilation Guide for nvm-rs

This guide explains how to build nvm-rs binaries for multiple platforms.

## Quick Start (Recommended)

### Using GitHub Actions (Easiest)

The included GitHub Actions workflow automatically builds for all platforms:

```bash
# Push to main/dev branch to trigger automatic builds
git push origin dev
```

Binaries will be available as artifacts in the Actions tab.

### Manual Cross-Compilation from Windows

**Note**: Cross-compilation from Windows may require additional setup.

#### Prerequisites

1. **Install Rustup** (if not already installed):

   ```bash
   # Download and install rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Or download from: https://rustup.rs/
   ```

2. **Add targets**:

   ```bash
   # For Linux x86_64
   rustup target add x86_64-unknown-linux-gnu

   # For macOS ARM64 (Apple Silicon)
   rustup target add aarch64-apple-darwin
   ```

#### Building

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS ARM64
cargo build --release --target aarch64-apple-darwin
# Binary: target/aarch64-apple-darwin/release/nvm

# Windows (native)
cargo build --release
# Binary: target/release/nvm.exe
```

## Alternative: Native Compilation

For more reliable builds, compile on the target platforms:

### Linux (using Docker)

```bash
# Build Linux binary using Docker
docker run --rm -v $(pwd):/app -w /app rust:latest cargo build --release

# The binary will be at: target/release/nvm
```

### Linux (using WSL)

```bash
# From WSL Ubuntu/Debian
sudo apt update
sudo apt install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
cargo build --release
```

### macOS ARM64 (Native)

```bash
# On Apple Silicon Mac
brew install rust
cargo build --release
```

## Platform-Specific Considerations

### Linux

- ✅ **Fully compatible**: No platform-specific code
- ✅ **Dependencies**: All crates are pure Rust
- ✅ **File paths**: Uses standard Unix paths

### macOS ARM64

- ✅ **Compatible**: Apple Silicon uses ARM64 architecture
- ✅ **Dependencies**: All crates work on ARM64
- ⚠️ **Testing**: May need testing on actual Apple Silicon hardware

### Windows (Current)

- ✅ **Working**: Current development platform
- ✅ **Windows-specific code**: Uses `#[cfg(windows)]` for symlink fallbacks

## Distribution

After building, you can distribute the binaries:

- **Linux**: `nvm` (statically linked, no dependencies)
- **macOS**: `nvm` (may need testing on macOS)
- **Windows**: `nvm.exe`

## CI/CD Integration

For automated builds, use the included GitHub Actions workflow in `.github/workflows/cross-platform.yml`.

## Troubleshooting

### Cross-compilation Issues

If cross-compilation fails on Windows:

1. **Update Rust**: `rustup update`
2. **Reinstall targets**:

   ```bash
   rustup target remove x86_64-unknown-linux-gnu aarch64-apple-darwin
   rustup target add x86_64-unknown-linux-gnu aarch64-apple-darwin
   ```

3. **Use native compilation**: Build on target platforms using Docker/WSL
4. **Use GitHub Actions**: Let CI handle cross-compilation

## Automated Cross-Compilation Script

Use the included PowerShell script for automated cross-compilation:

```powershell
.\scripts\cross-compile.ps1
```
