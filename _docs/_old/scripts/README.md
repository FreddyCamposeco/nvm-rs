# Scripts Directory

This directory contains utility scripts for the nvm-rs project.

## Available Scripts

### `cross-compile.ps1`

Automated cross-compilation script for building nvm-rs binaries for multiple platforms.

**Usage:**

```powershell
.\scripts\cross-compile.ps1
```

**What it does:**

- Installs Rust targets for Linux and macOS
- Attempts to build for Linux x86_64 and macOS ARM64
- Provides alternative options if cross-compilation fails
- Works from any directory

### `test-i18n.bat`

Test script for internationalization functionality.

**Usage:**

```cmd
scripts\test-i18n.bat
```

**What it does:**

- Tests English and Spanish translations
- Verifies NVM_LANG environment variable functionality
- Tests the `lang` command
- Automatically changes to project root directory

### `nvm-lang-setup.ps1`

PowerShell configuration examples for NVM_LANG environment variable.

**Usage:**

```powershell
# View the file for configuration examples
Get-Content scripts\nvm-lang-setup.ps1
```

**What it contains:**

- Examples for setting NVM_LANG permanently
- PowerShell profile configuration
- Usage examples

## Running Scripts

All scripts are designed to work from any directory within the project. They automatically navigate to the correct locations and return to the original directory when finished.

## Requirements

- **PowerShell**: For `.ps1` scripts (Windows built-in)
- **CMD**: For `.bat` scripts (Windows built-in)
- **Rust**: For cross-compilation script
- **Rustup**: For target management
