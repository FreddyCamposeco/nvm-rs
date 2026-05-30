use anyhow::Result;
use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use std::env;
use std::path::PathBuf;

mod config;
mod core;
mod error;
mod i18n;
mod utils;
mod commands;
mod platform;

use config::Config;
use crate::core::nvm_config::NvmConfig;
use i18n::{set_locale, Locale};

#[derive(Parser)]
#[command(name = "nvm")]
#[command(
    version,
    about = "Node Version Manager - Rust Edition",
    long_about = None,
    disable_version_flag = true
)]
struct Cli {
    /// Print version information
    #[arg(short = 'v', short_alias = 'V', long = "version", action = ArgAction::SetTrue)]
    version: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a Node.js version
    Install {
        /// Version to install (e.g., 18.19.0, lts, latest)
        version: String,
        /// Do not switch to the installed version automatically
        #[arg(long)]
        no_use: bool,
    },

    /// Uninstall a Node.js version
    Uninstall {
        /// Version to uninstall
        version: String,
        /// Force uninstall even if it's the current version
        #[arg(long)]
        force: bool,
    },

    /// Switch to a Node.js version
    Use {
        /// Version to use (optional, will check .nvmrc or default)
        version: Option<String>,
    },

    /// List installed versions
    #[command(alias = "list")]
    Ls,

    /// List remote versions available for download
    LsRemote {
        /// Show only LTS versions
        #[arg(long)]
        lts: bool,
    },

    /// Show current active version
    Current,

    /// Create an alias for a version
    Alias {
        /// Alias name
        name: String,
        /// Version to alias
        version: String,
    },

    /// Remove an alias
    Unalias {
        /// Alias name to remove
        name: String,
    },

    /// List all aliases
    Aliases,

    /// Show path to the node binary for a version
    Which {
        /// Version to locate (default: current)
        version: Option<String>,
    },

    /// Run a command using a specific Node.js version
    Run {
        /// Node.js version to use
        version: String,
        /// Command and arguments to run
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },

    /// Run a command using a specific Node.js version (alias for run)
    Exec {
        /// Node.js version to use
        version: String,
        /// Command and arguments to run
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },

    /// Enable symlink support (Windows only - requires admin rights)
    #[cfg(windows)]
    EnableSymlinks,

    /// Verify installation and system info
    Doctor {
        /// Show all Node.js installations found in the system
        #[arg(long)]
        all: bool,
        /// Show only system Node.js (not NVM-managed)
        #[arg(long)]
        system: bool,
        /// Fix PATH and shell configuration (Unix)
        #[arg(long)]
        fix: bool,
    },

    /// Remove unused versions (keep current and LTS)
    Cleanup {
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Clear the remote and installed version caches
    CacheClear,

    /// Update nvm itself
    #[cfg(feature = "self-update")]
    SelfUpdate,

    /// Set default version for new shells
    SetDefault {
        /// Version to set as default (e.g., 18.19.0, lts, latest)
        version: String,
    },

    /// Set language/locale
    Lang {
        /// Locale code (en, es)
        locale: String,
    },

    /// Install nvm from GitHub releases
    InstallSelf {
        /// Version to install (e.g., v0.1.0, latest)
        #[arg(short, long)]
        version: Option<String>,
        /// Install directory (default: auto-detect)
        #[arg(short, long)]
        dir: Option<PathBuf>,
        /// Include self-update capability
        #[arg(long)]
        with_self_update: bool,
    },

    /// Uninstall nvm from the system
    UninstallSelf {
        /// Installation directory (default: auto-detect)
        #[arg(short, long)]
        dir: Option<PathBuf>,
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
        /// Remove all data (Node.js versions, cache, config)
        #[arg(long)]
        purge: bool,
        /// Remove shell configuration entries
        #[arg(long)]
        remove_config: bool,
    },

    /// Update nvm to the latest version
    UpdateSelf {
        /// Target version (default: latest)
        #[arg(short, long)]
        version: Option<String>,
        /// Include self-update capability
        #[arg(long)]
        with_self_update: bool,
    },

    /// Show installation statistics
    Stats {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }

    #[cfg(unix)]
    {
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let msg = info.to_string();
            if msg.contains("Broken pipe") {
                std::process::exit(0);
            }
            default_hook(info);
        }));
    }

    // Initialize colors early so locale messages are styled
    utils::init_colors();

    // Determine config path before full Config::new() so we can load locale
    let nvm_dir = if let Ok(home_var) = env::var("NVM_HOME") {
        PathBuf::from(home_var)
    } else {
        home::home_dir()
            .map(|h| h.join(".nvm"))
            .unwrap_or_else(|| PathBuf::from(".nvm"))
    };

    // Load locale: config file > NVM_LANG env var > default English
    let nvm_config = NvmConfig::load(&nvm_dir.join("config.json"));
    let locale_str = nvm_config.locale
        .or_else(|| env::var("NVM_LANG").ok())
        .unwrap_or_else(|| "en".to_string());
    let locale = Locale::from_str(&locale_str).unwrap_or(Locale::En);
    set_locale(locale);

    let cli = Cli::parse();
    let config = Config::new()?;

    if cli.version {
        println!("nvm {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    match cli.command {
        None => {
            Cli::command().print_help()?;
            println!();
        }

        Some(Commands::Install { version, no_use }) => {
            commands::install::install(&version, no_use, &config).await?;
        }

        Some(Commands::Uninstall { version, force }) => {
            commands::uninstall::uninstall(&version, force, &config).await?;
        }

        Some(Commands::Use { version }) => {
            commands::use_version::use_version(version, &config).await?;
        }

        Some(Commands::Ls) => {
            commands::list::list_installed(&config).await?;
        }

        Some(Commands::LsRemote { lts }) => {
            commands::list::list_remote(lts, &config).await?;
        }

        Some(Commands::Current) => {
            commands::list::show_current(&config)?;
        }

        Some(Commands::Alias { name, version }) => {
            commands::alias::create_alias(name, version, &config).await?;
        }

        Some(Commands::Unalias { name }) => {
            commands::alias::remove_alias(name)?;
        }

        Some(Commands::Aliases) => {
            commands::alias::list_aliases()?;
        }

        Some(Commands::Which { version }) => {
            commands::which::which(version, &config).await?;
        }

        Some(Commands::Run { version, command }) => {
            commands::run::run_with_version(&version, &command, &config).await?;
        }

        Some(Commands::Exec { version, command }) => {
            commands::run::run_with_version(&version, &command, &config).await?;
        }

        #[cfg(windows)]
        Some(Commands::EnableSymlinks) => {
            commands::misc::enable_symlinks()?;
        }

        #[cfg(feature = "self-update")]
        Some(Commands::SelfUpdate) => {
            commands::misc::self_update()?;
        }

        Some(Commands::Doctor { all, system, fix }) => {
            if all || system {
                if system || all {
                    commands::doctor::show_system_node();
                }
                if all {
                    commands::doctor::show_all_installations();
                }
            }

            if !all && !system {
                commands::doctor::run_diagnostics(&config, fix)?;
            }
        }

        Some(Commands::Cleanup { yes }) => {
            commands::misc::cleanup(yes, &config).await?;
        }

        Some(Commands::CacheClear) => {
            commands::misc::cache_clear(&config)?;
        }

        Some(Commands::SetDefault { version }) => {
            commands::misc::set_default(version, &config)?;
        }

        Some(Commands::Lang { locale }) => {
            commands::misc::set_language(locale, &config)?;
        }

        Some(Commands::InstallSelf { version, dir, with_self_update }) => {
            commands::self_management::install_self(version, dir, with_self_update).await?;
        }

        Some(Commands::UninstallSelf { dir, yes, purge, remove_config }) => {
            commands::self_management::uninstall_self(dir, yes, purge, remove_config)?;
        }

        Some(Commands::UpdateSelf { version, with_self_update }) => {
            commands::self_management::update_self(version, with_self_update).await?;
        }

        Some(Commands::Stats { json }) => {
            let stats = commands::stats::get_stats(&config).await?;
            if json {
                commands::stats::display_stats_json(&stats)?;
            } else {
                commands::stats::display_stats(&stats);
            }
        }
    }

    Ok(())
}
