# Release Notes - v0.5.1

**Release Date:** December 8, 2025
**Type:** Maintenance Release
**Status:** ✅ Production Ready

## 📋 Overview

This is a maintenance release focused on repository organization, infrastructure improvements, and enhanced documentation. No functional changes were made to the core application.

## 🔧 Changes

### 🗂️ Repository Reorganization

- **Scripts Organization**: All automation scripts have been reorganized into logical categories:
  - `scripts/build/` - Compilation and build scripts
  - `scripts/install/` - Installation and uninstallation scripts
  - `scripts/release/` - Release validation and publishing scripts
- **Documentation Centralization**: All project documentation moved to `docs/` directory
- **Repository Cleanup**: Removed obsolete files, temporary artifacts, and outdated documentation

### 📝 Documentation Improvements

- **Category-specific READMEs**: Each script category now has comprehensive documentation
  - `scripts/build/README.md` - Build process documentation
  - `scripts/install/README.md` - Installation guide
  - `scripts/release/README.md` - Release workflow documentation
- **Documentation Index**: Added `docs/README.md` as navigation hub
- **Updated References**: All Makefile targets and README URLs updated to reflect new structure

### 🐛 Bug Fixes

- **install.ps1**: Improved asset detection with flexible pattern matching
  - Enhanced fallback mechanisms for finding release binaries
  - Better handling of versioned assets

### 🧹 Infrastructure

- **Makefile**: Updated all script paths to match new organization
- **Git History**: Clean, descriptive commit messages following conventional commits
- **Code Quality**: All scripts validated for syntax and functionality

## 📦 Artifacts

### Windows

- **nvm-v0.5.1-windows-x64.exe** - Windows 64-bit executable
  - Size: 4.24 MB
  - SHA256: `11719B494358D13C85C5E8ED9978656C92D401EC89FC03E7424A05AACC9FCD19`

### Linux & macOS

Linux and macOS builds will be added in a future release.

## 📥 Installation

### Windows (PowerShell)

```powershell
irm https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.1/install.ps1 | iex
```

### Manual Installation

1. Download `nvm-v0.5.1-windows-x64.exe`
2. Rename to `nvm.exe`
3. Place in desired directory (e.g., `C:\Program Files\nvm`)
4. Add directory to system PATH

## ✅ Verification

After installation, verify with:

```bash
nvm --version
# Should output: 0.5.1

nvm doctor --all
# Verifies complete installation
```

## 🔄 Upgrade Instructions

If upgrading from v0.5.0:

```powershell
# Using the installer (recommended)
irm https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.1/install.ps1 | iex -WithSelfUpdate

# Or manually replace the executable
```

**Note:** v0.5.1 is a drop-in replacement for v0.5.0. All configuration and installed Node.js versions remain intact.

## 📊 What's Changed

### Commits in this Release

- `chore: cleanup repository structure and organize documentation`
- `refactor: organize scripts into logical categories (build/, install/, release/)`
- `refactor: update Makefile with new script paths`
- `docs: update installation URLs with new script paths`
- `fix: update install script to find nvm.exe in release assets`

### Repository Stats

- **Scripts Reorganized**: 11 scripts moved to category directories
- **Documentation Created**: 4 new README files (~1,200 lines)
- **Files Cleaned**: 47 obsolete files removed
- **Commits**: 5 descriptive commits

## 🎯 Compatibility

- **Operating Systems**: Windows 10/11 (x64)
- **PowerShell**: 5.1+ or PowerShell Core 7+
- **Dependencies**: None (standalone binary)

## 🔗 Links

- **Repository**: <https://github.com/FreddyCamposeco/nvm-rs>
- **Issues**: <https://github.com/FreddyCamposeco/nvm-rs/issues>
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)

## 🙏 Acknowledgments

Thank you to everyone who contributed to making nvm-rs better!

## 📝 Notes

- This is a maintenance release with no breaking changes
- Binary size remains consistent with v0.5.0 (~4.2 MB)
- All features from v0.5.0 are preserved
- Repository structure significantly improved for contributors

---

**Full Changelog**: <https://github.com/FreddyCamposeco/nvm-rs/compare/v0.5.0...v0.5.1>
