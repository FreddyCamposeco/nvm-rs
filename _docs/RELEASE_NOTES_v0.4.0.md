# Release Notes v0.4.0

**Release Date**: December 7, 2025
**Tag**: v0.4.0
**Previous Version**: v0.3.0

## ✨ New Features

### 1. System Node.js Detection Module

Complete module for detecting Node.js installations on the system.

**File**: `src/core/detection.rs` (313 lines)

**Key Functions**:

- `detect_system_node()` - Find first Node.js in PATH or system locations
- `find_all_node_installations()` - Locate all Node.js installations
- `find_node_in_path()` - Search using `where` (Windows) or `which` (Unix)
- `find_node_in_system_locations()` - Check Program Files, /usr/local, ~/.local

**Data Structures**:

- `SystemNodeInfo` struct: path, version, npm_version, source
- `DetectionSource` enum: PathEnvironment, SystemInstallation, NvmManaged

**Platform Support**: Windows, Linux, macOS

### 2. Cache Improvements

Extended cache duration and added comprehensive metadata.

**Changes**:

- Cache duration: **15 minutes → 24 hours** (1440 minutes)
- New `CacheInfo` struct with complete metadata
- Human-readable formatting methods

**New Functions**:

- `get_cache_info()` - Detailed cache information
- `get_cache_total_size()` - Recursive directory size calculation
- `size_human_readable()` - Convert bytes to KB/MB/GB
- `last_updated_human_readable()` - Format elapsed time

**Location**: `src/core/cache.rs` (+162 lines)

### 3. Stats Command

New command to show installation statistics.

**Usage**:

```bash
nvm stats          # Formatted output with colors
nvm stats --json   # JSON export for scripting
```

**Collected Metrics** (10 total):

- NVM version and location
- Installation size
- Number of installed versions
- Active version
- Total Node.js size
- Aliases count
- Cache size and status
- Cache age

**File**: `src/commands/stats.rs` (256 lines)

**Key Functions**:

- `get_stats()` - Gather all statistics (async)
- `display_stats()` - Formatted output with ANSI colors
- `display_stats_json()` - JSON serialization
- `get_active_version()` - Read current symlink/junction
- `calculate_dir_size()` - Recursive size calculation
- `format_size()` - Human-readable size formatting
- `format_age()` - Human-readable time formatting

## 📊 Statistics

- **Lines of code**: +569
- **Files created**: 3
  - `src/core/detection.rs` (313 lines)
  - `src/commands/stats.rs` (256 lines)
  - `src/commands/mod.rs` (module)
- **Files modified**: 3
  - `src/main.rs` - CLI integration
  - `src/config.rs` - Cache duration update
  - `src/core/cache.rs` - Extended functionality
- **Commits**: 4
- **Binary size**: 4.05 MB (unchanged)

## 🛠️ Technical Details

### Module Organization

New `commands/` module structure for future expansions:

```
src/
├── commands/
│   ├── mod.rs
│   └── stats.rs
└── core/
    ├── detection.rs  (NEW)
    └── cache.rs      (EXTENDED)
```

### Dependencies

No new dependencies added. Uses existing:

- `colored` - ANSI color output
- `serde_json` - JSON serialization
- `tokio` - Async runtime

### Cross-Platform Support

All three features are fully cross-platform:

- Windows: Uses `where` command, checks Program Files
- Linux: Uses `which`, checks /usr/local, ~/.local
- macOS: Uses `which`, checks /usr/local, Homebrew locations

## 🚀 Installation

### Using nvm (if already installed)

```bash
nvm update-self         # Update to latest
nvm install-self -v v0.4.0  # Install specific version
```

### Download Binary

Download the appropriate binary for your platform from the [Releases page](https://github.com/FreddyCamposeco/nvm-rs/releases/tag/v0.4.0).

## 📝 Breaking Changes

None. This release is fully backward compatible with v0.3.0.

## 🔄 Upgrade Notes

Simply update using:

```bash
nvm update-self
```

All existing configurations, aliases, and installed Node.js versions remain intact.

## 🎯 Future Plans

- Integration with `nvm doctor --all` (**COMPLETED in v0.5.0**)
- Configuration from file (nvm.toml/settings.json)
- Plugin system
- Cache management commands

## 📚 Documentation

- [CHANGELOG.md](CHANGELOG.md) - Full changelog
- [README.md](README.md) - Complete documentation

## 🙏 Credits

Thanks to all contributors and the Rust community for their support!

**Full Changelog**: [v0.3.0...v0.4.0](https://github.com/FreddyCamposeco/nvm-rs/compare/v0.3.0...v0.4.0)
