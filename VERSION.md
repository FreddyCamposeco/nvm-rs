# Version Information

## Current Version

**v0.7.0** (2026-05-31)

## Version History

### v0.7.0 - May 31, 2026

#### 🚀 Shell Integration & Package Migration

**Added:**
- `nvm shell-init <shell>` — auto `.nvmrc` detection on `cd` (bash, zsh, fish, powershell)
- `nvm reinstall-packages <version>` — migrate global npm packages between versions
- GitHub Actions release workflow with multi-platform builds and SHA256 checksums

**Fixed:**
- `nvm uninstall --force` now removes `.nvm-version` alongside the symlink

**Status**: Production Ready

---

### v0.6.1 - February 11, 2026

#### 📦 Release: CLI Flags & Help

**Improvements:**
- ✅ Added `-v` alias for version output (`-V` kept)
- ✅ Help output shows version flag consistently
- ✅ No-argument invocation shows help
- ✅ Build cleanup: fix `doctor` flag parameter usage
- ✅ Avoid broken pipe panic when output is piped

**Status**: Production Ready - Minor CLI Enhancements

---

### v0.6.0 - February 2, 2026

#### 📦 Release: Code Cleanup & Repository Reorganization

**Improvements:**
- ✅ Removed temporary documentation files
- ✅ Consolidated release notes into single CHANGELOG.md
- ✅ Cleaned up repository structure
- ✅ Simplified documentation organization

**Status**: Production Ready - Code Quality Focus

---

### v0.5.1 - December 9, 2025

#### 🔧 Phase 4: Typed Error Handling & Code Cleanup

**Error Handling:**
- ✅ Typed error handling with `thiserror`
- ✅ 8+ custom error types
- ✅ Improved error context

**Code Quality:**
- ✅ Removed 4 unused methods
- ✅ Fixed 12 clippy warnings
- ✅ Zero compiler warnings
- ✨ Scripts organized by categories (build/, install/, release/)
- Status: **PRODUCTION READY**

### v0.5.0 - December 7, 2025

- ✅ System Node.js Detection integration in `doctor` command
- ✅ Code cleanup: 0 warnings final compilation
- ✅ `nvm doctor --all` / `nvm doctor --system`
- Status: **PRODUCTION READY**

### v0.4.0 - December 7, 2025

- ✅ System Node.js Detection module (313 lines)
- ✅ Cache improvements (15 min → 24 hours)
- ✅ Stats command with JSON export
- Status: **PRODUCTION READY**

### v0.3.0 - December 6, 2025

- ✅ Full uninstall cleanup feature
- Status: **PRODUCTION READY**

## Platform Support

| Platform | Status | Architecture |
|----------|--------|--------------|
| Windows | ✅ | x64, x86, ARM64 |
| Linux | ✅ | x64, ARM64 |
| macOS | ✅ | x64, ARM64 (Apple Silicon) |

## Build Information

- **Language**: Rust 2021 Edition
- **Build System**: Cargo
- **Release Binary**: nvm.exe (Windows), nvm (Unix)
- **Binary Size**: ~4.05 MB (stripped)

## Compilation Status

- **Latest Build**: v0.5.1 (Phase 4 Complete)
- **Compilation Time**: ~0.8s (debug), ~8s (release with LTO)
- **Errors**: 0 ✅
- **Warnings**: 0 ✅
- **Tests**: All passing ✅
- **Binary Size**: ~4.13 MB (release, stripped)

## Release Notes URL

GitHub Releases: [https://github.com/FreddyCamposeco/nvm-rs/releases](https://github.com/FreddyCamposeco/nvm-rs/releases)
