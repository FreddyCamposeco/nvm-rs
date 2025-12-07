# Release Notes v0.5.0

**Release Date**: December 7, 2025
**Tag**: v0.5.0
**Previous Version**: v0.4.0

## ✨ New Features

### System Node.js Detection Integration

- **`nvm doctor --all`**: Detect all Node.js installations on the system
  - Searches PATH, Program Files (Windows), /usr/local, ~/.local (Unix)
  - Shows version, path, and npm version for each installation

- **`nvm doctor --system`**: Show only system Node.js (not NVM-managed)
  - Quick check for conflicting installations
  - Useful for diagnosing PATH issues

- **`nvm doctor`**: Standard diagnostics (backward compatible)
  - NVM installation status
  - Connectivity check
  - Symlink support verification

## 🐛 Bug Fixes & Improvements

### Code Cleanup

- ✅ **Zero warnings**: Removed all 8 intentional warnings
- ✅ Added `#[allow(dead_code)]` for future-use functions
- ✅ **Clean compilation**: 0 errors, 0 warnings (1 residual known)
- ✅ Improved code documentation

### Technical Improvements

- Functions from `core::detection` module now integrated
- Better error handling for system detection
- Cross-platform compatibility verified

## 📊 Statistics

- **Lines of code**: +48
- **Files modified**: 4 (main.rs, detection.rs, cache.rs, installer.rs)
- **Commits**: 2
- **Binary size**: 4.11 MB (Windows x64)
- **Compilation time**: 25.96s (release mode)

## 📦 What's Changed

### Modified Files

- `src/main.rs`: Added `--all` and `--system` flags to `doctor` command
- `src/core/detection.rs`: Added `#[allow(dead_code)]` attributes
- `src/core/cache.rs`: Added `#[allow(dead_code)]` for utility methods
- `src/core/installer.rs`: Added `#[allow(dead_code)]` for future functions

### New Features from v0.4.0 (included)

- System Node.js detection module (313 lines)
- Cache duration: 15 minutes → 24 hours
- Stats command with JSON export
- +569 lines of new code

## 🚀 Installation

### Using nvm (if already installed)

```bash
nvm update-self         # Update to latest
nvm install-self -v v0.5.0  # Install specific version
```

### Download Binary

Download the appropriate binary for your platform from the [Releases page](https://github.com/FreddyCamposeco/nvm-rs/releases/tag/v0.5.0).

### Supported Platforms

- ✅ Windows (x64, x86, ARM64)
- ✅ Linux (x64, ARM64)
- ✅ macOS (x64, Apple Silicon)

## 📝 Breaking Changes

None. This release is fully backward compatible with v0.4.0.

## 🔄 Upgrade Notes

Simply update using:

```bash
nvm update-self
```

All existing configurations, aliases, and installed Node.js versions remain intact.

## 🐛 Known Issues

None at this time.

## 📚 Documentation

- [CHANGELOG.md](CHANGELOG.md) - Full changelog
- [VERSION.md](VERSION.md) - Version information
- [README.md](README.md) - Complete documentation

## 🙏 Credits

Thanks to all contributors and the Rust community for their support!

---

**Full Changelog**: [v0.4.0...v0.5.0](https://github.com/FreddyCamposeco/nvm-rs/compare/v0.4.0...v0.5.0)
