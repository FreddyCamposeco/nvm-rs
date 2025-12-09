# Build nvm-rs on Linux

## Prerequisites

### Automatic Setup (Recommended)

The easiest way to set up your Linux environment is to use the provided setup script:

```bash
sudo bash ./scripts/setup-linux-build-env.sh
```

This script will:
- Detect your Linux distribution
- Install all required dependencies
- Verify the installation

### Supported Distributions

| Distribution | Package Manager | Setup Command |
|---|---|---|
| Ubuntu / Debian | apt | `sudo bash ./scripts/setup-linux-build-env.sh` |
| Fedora / RHEL | dnf/yum | `sudo bash ./scripts/setup-linux-build-env.sh` |
| Arch Linux | pacman | `sudo bash ./scripts/setup-linux-build-env.sh` |
| Alpine Linux | apk | `sudo bash ./scripts/setup-linux-build-env.sh` |

### Manual Installation

If you prefer to install dependencies manually:

#### Ubuntu / Debian

```bash
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev build-essential curl
```

#### Fedora / RHEL

```bash
sudo dnf install -y pkgconfig openssl-devel gcc make curl
# or for older systems
sudo yum install -y pkgconfig openssl-devel gcc make curl
```

#### Arch Linux

```bash
sudo pacman -Sy pkg-config openssl base-devel curl
```

#### Alpine Linux

```bash
sudo apk add pkgconfig openssl-dev build-base curl
```

## Building nvm-rs

### Quick Build

```bash
bash ./scripts/build/build.sh
```

### Building Specific Targets

```bash
# Linux x64
bash ./scripts/build/build.sh --target x86_64-unknown-linux-gnu

# Linux ARM64
bash ./scripts/build/build.sh --target aarch64-unknown-linux-gnu

# With self-update capability
bash ./scripts/build/build.sh --with-self-update
```

### Using Cargo Directly

```bash
# Release build
cargo build --target x86_64-unknown-linux-gnu --release

# Debug build
cargo build --target x86_64-unknown-linux-gnu

# Optimized for specific CPU
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Available Targets

To see all available Rust targets:

```bash
rustup target list
```

Common targets for Linux:
- `x86_64-unknown-linux-gnu` - Linux x64 (glibc)
- `x86_64-unknown-linux-musl` - Linux x64 (musl/static)
- `aarch64-unknown-linux-gnu` - Linux ARM64 (glibc)
- `aarch64-unknown-linux-musl` - Linux ARM64 (musl/static)
- `armv7-unknown-linux-gnueabihf` - Linux ARMv7 (glibc)

## Output

Built binaries will be in `release-builds/` directory:

```bash
ls -lh release-builds/
# Example output:
# nvm-v0.5.1-linux-gnu-x64
# nvm-v0.5.1-linux-musl-x64
# CHECKSUMS.sha256
# manifest.json
```

## Testing

### Test the build

```bash
# Make binary executable
chmod +x ./release-builds/nvm-*

# Run version command
./release-builds/nvm-v0.5.1-linux-gnu-x64 --version

# Test help
./release-builds/nvm-v0.5.1-linux-gnu-x64 --help
```

### Install locally for testing

```bash
# Create installation directory
mkdir -p ~/.local/bin

# Copy binary
cp ./release-builds/nvm-v0.5.1-linux-gnu-x64 ~/.local/bin/nvm

# Make executable
chmod +x ~/.local/bin/nvm

# Test
~/.local/bin/nvm --version
```

## Troubleshooting

### Error: Could not find OpenSSL

**Problem:**
```
error: failed to run custom build command for `openssl-sys v0.9.111`
Could not find directory of OpenSSL installation
```

**Solution:**
```bash
# Install setup script
sudo bash ./scripts/setup-linux-build-env.sh

# Or install manually:
# Ubuntu/Debian
sudo apt-get install libssl-dev

# Fedora/RHEL
sudo dnf install openssl-devel

# Arch
sudo pacman -S openssl

# Alpine
sudo apk add openssl-dev
```

### Error: pkg-config not found

**Problem:**
```
The pkg-config command could not be found.
```

**Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install pkg-config

# Fedora/RHEL
sudo dnf install pkgconfig

# Arch
sudo pacman -S pkg-config

# Alpine
sudo apk add pkgconfig
```

### Error: C compiler not found

**Problem:**
```
error: no default toolchain configured
```

**Solution:**
```bash
# Install build tools
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc make

# Arch
sudo pacman -S base-devel

# Alpine
sudo apk add build-base
```

### Slow build compilation

The first build can take several minutes. To speed up subsequent builds:

```bash
# Use incremental compilation (enabled by default in dev mode)
cargo build --target x86_64-unknown-linux-gnu

# For faster release builds with optimizations
CARGO_BUILD_JOBS=4 cargo build --target x86_64-unknown-linux-gnu --release
```

### Build cache issues

If you encounter build cache issues:

```bash
# Clean build
cargo clean
bash ./scripts/build/build.sh
```

## Performance Tips

### Faster Builds

```bash
# Parallel compilation
export CARGO_BUILD_JOBS=$(nproc)

# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache

# Then build
cargo build --release
```

### Smaller Binaries

```bash
# Strip debug symbols
cargo build --release
strip ./target/x86_64-unknown-linux-gnu/release/nvm

# Or use cargo-strip
cargo install cargo-strip
cargo strip --release
```

## Next Steps

After building:

1. **Test the binary:** `./release-builds/nvm-v0.5.1-linux-gnu-x64 --help`
2. **Run validation:** `bash ./scripts/release/validate-release.ps1` (requires PowerShell)
3. **Install:** Copy to `~/.local/bin/` or system PATH
4. **Verify:** Run `nvm --version` and `nvm doctor --all`

## Additional Resources

- [Rust Build Performance](https://doc.rust-lang.org/cargo/build-cache.html)
- [OpenSSL Build Issues](https://github.com/sfackler/rust-openssl/blob/master/openssl-sys/build/main.rs)
- [nvm-rs Repository](https://github.com/FreddyCamposeco/nvm-rs)
