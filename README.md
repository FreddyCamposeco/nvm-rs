# nvm-rs

Node Version Manager written in Rust, inspired by nvm-windows.

## Features

- ✅ Install Node.js versions (simulated)
- ✅ Switch between versions (with Windows compatibility)
- ✅ List installed and remote versions
- ✅ Cross-platform support (Windows, Linux, macOS)
- ✅ Command-line interface
- ✅ Directory management and version switching
- ✅ Internationalization support (English/Spanish)

## Commands

- `nvm install <version>` - Install a Node.js version (creates dummy files)
- `nvm uninstall <version>` - Uninstall a Node.js version
- `nvm use <version>` - Switch to a Node.js version
- `nvm ls` - List installed versions
- `nvm ls-remote` - List available versions (shows hardcoded list)
- `nvm current` - Show current version
- `nvm version` - Show nvm version
- `nvm help` - Show help
- `nvm doctor` - Show system information
- `nvm lang <locale>` - Set language (en/es)

## Internationalization

This project supports multiple languages through an external file-based translation system:

- **English** (`en`) - Default language
- **Spanish** (`es`) - Español

### Environment Variables (Portable)

The project uses environment variables for language configuration, which are **fully portable** across Windows, Linux, and macOS:

- **Advantages**: Standard across all platforms, no additional configuration files needed, can be set per-session or globally
- **Persistence**: Variables set in shell profiles (`.bashrc`, `.zshrc`, PowerShell profile) persist across sessions
- **Override**: Can be temporarily overridden for specific commands

### Language Detection

The language is automatically detected from the `NVM_LANG` environment variable:

- `NVM_LANG=es` or `NVM_LANG=es_ES` → Spanish
- `NVM_LANG=en` or `NVM_LANG=en_US` → English
- Default: English (if NVM_LANG is not set)

### Manual Language Switching

You can manually set the language using the `lang` command:

```bash
# Set to English
nvm lang en

# Set to Spanish
nvm lang es
```

Or set the environment variable:

```bash
# Windows
$env:NVM_LANG = "es"
nvm --help

# Linux/macOS
export NVM_LANG=es
nvm --help
```

See `scripts/nvm-lang-setup.ps1` for PowerShell configuration examples.

### Translation Files

Translations are stored in YAML files in the `locales/` directory:

- `locales/en.yaml` - English translations
- `locales/es.yaml` - Spanish translations

YAML format allows for better structure and comments. Each translation is a key-value pair.

### Adding New Languages

To add a new language:

1. Create a new file `locales/<locale>.yaml`
2. Add all translation keys from existing YAML files
3. Update the `Locale` enum in `src/i18n.rs`
4. Add the new locale to the `from_str` and `file_name` methods

Example YAML structure:

```yaml
# French translations
nvm_version: "nvm v{}"
unknown_command: "Commande inconnue: {}"
# ... etc
```

## Building

```bash
cargo build --release
```

The binary will be named `nvm` (not `nvm-rs`).

### Cross-Compilation

This project supports cross-compilation to multiple platforms. See [CROSS-COMPILE.md](CROSS-COMPILE.md) for detailed instructions on building for Linux and macOS ARM.

**Note**: Cross-compiling from Windows may require additional setup. For reliable builds, consider:

- **GitHub Actions**: Automatic builds for all platforms (recommended)
- **Docker**: `docker run --rm -v $(pwd):/app -w /app rust:latest cargo build --release`
- **WSL**: Native compilation on Ubuntu WSL
- **Native platforms**: Build directly on Linux/macOS

Supported targets:

- **Linux x86_64**: `x86_64-unknown-linux-gnu`
- **macOS ARM64**: `aarch64-apple-darwin`
- **Windows x86_64**: Native (current development platform)

## Usage

After building, you can run:

```bash
./target/release/nvm help
```

## Demo

```powershell
# Install a version
.\target\release\nvm.exe install 18.17.0
# Output: Installing Node.js v18.17.0...
#         ✓ Installed Node.js v18.17.0 (simulated)

# List installed versions
.\target\release\nvm.exe ls
# Output: Installed versions:
#           v18.17.0

# Switch to version
.\target\release\nvm.exe use 18.17.0
# Output: ✓ Now using node v18.17.0

# Check current version
.\target\release\nvm.exe current
# Output: v18.17.0
```

### Internationalization Demo

Run the included test script to see internationalization in action:

```cmd
# On Windows
scripts\test-i18n.bat

# Or manually test:
set NVM_LANG=en && nvm --help
set NVM_LANG=es && nvm --help
```

The translations are now stored in YAML format for better maintainability.

## Project Structure

```text
src/
├── main.rs      # CLI entry point with argument parsing
├── config.rs    # Configuration management
├── versions.rs  # Version management logic
├── install.rs   # Installation/uninstallation logic
├── utils.rs     # Utility functions
└── i18n.rs      # Internationalization system
locales/
├── en.yaml      # English translations
└── es.yaml      # Spanish translations
scripts/
├── cross-compile.ps1    # Cross-compilation automation script
├── test-i18n.bat        # Internationalization test script
├── nvm-lang-setup.ps1   # PowerShell configuration examples
└── README.md            # Scripts documentation
CROSS-COMPILE.md         # Cross-compilation guide
```

## Current Status

✅ **Working MVP**: The project has a fully functional command-line interface that works on Windows!

- **Core functionality**: All basic commands work correctly
- **Windows compatibility**: Handles symlink limitations with directory copying fallback
- **Error handling**: Proper error messages and validation
- **Minimal dependencies**: Uses only `anyhow`, `lazy_static`, and `yaml-rust2` for error handling, static initialization, and YAML parsing
- **Internationalization**: External file-based translation system supporting English and Spanish via `NVM_LANG` environment variable
- **Cross-platform ready**: Code is compatible with Linux and macOS ARM compilation

## Next Steps

- [ ] Real Node.js downloads from nodejs.org
- [ ] Archive extraction (tar.gz/zip)
- [ ] Alias management
- [ ] Auto-version switching (.nvmrc support)
- [ ] Better Windows symlink support (with elevated privileges)
- [ ] Cross-platform testing on Linux and macOS
- [ ] Unit tests
- [ ] Additional language support

## Based on nvm-windows

This project is a Rust port of [nvm-windows](https://github.com/coreybutler/nvm-windows), maintaining similar functionality and command structure while providing:

- Better performance with Rust
- Native binary (no PowerShell dependency)
- Cross-platform compatibility
- More maintainable codebase
