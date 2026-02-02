# Version Information

## Current Version

**v0.6.0** (2026-02-02)

## Version History

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
