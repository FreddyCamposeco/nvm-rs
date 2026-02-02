use anyhow::Result;
use clap::{Parser, Subcommand};
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
use i18n::{set_locale, Locale};

#[derive(Parser)]
#[command(name = "nvm")]
#[command(version, about = "Node Version Manager - Rust Edition", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a Node.js version
    Install {
        /// Version to install (e.g., 18.19.0, lts, latest)
        version: String,
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

    /// Update nvm itself
    #[cfg(feature = "self-update")]
    SelfUpdate,

    /// Set default version for new shells
    SetDefault {
        /// Version to set as default
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
        // Evitar panic por broken pipe cuando el output se corta (ej. con `head`)
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }

    // Initialize locale from environment
    let nvm_lang = env::var("NVM_LANG").unwrap_or_else(|_| "en".to_string());
    let locale = Locale::from_str(&nvm_lang).unwrap_or(Locale::En);
    set_locale(locale);

    // Initialize colors
    utils::init_colors();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Create configuration
    let config = Config::new()?;

    // Execute command
    match cli.command {
        Commands::Install { version } => {
            commands::install::install(&version, &config).await?;
        }

        Commands::Uninstall { version, force } => {
            commands::uninstall::uninstall(&version, force, &config).await?;
        }

        Commands::Use { version } => {
            commands::use_version::use_version(version, &config).await?;
        }

        Commands::Ls => {
            commands::list::list_installed(&config).await?;
        }

        Commands::LsRemote { lts } => {
            commands::list::list_remote(lts, &config).await?;
        }

        Commands::Current => {
            commands::list::show_current(&config)?;
        }

        Commands::Alias { name, version } => {
            commands::alias::create_alias(name, version, &config).await?;
        }

        Commands::Unalias { name } => {
            commands::alias::remove_alias(name)?;
        }

        Commands::Aliases => {
            commands::alias::list_aliases()?;
        }

        #[cfg(windows)]
        Commands::EnableSymlinks => {
            commands::misc::enable_symlinks()?;
        }

        #[cfg(feature = "self-update")]
        Commands::SelfUpdate => {
            commands::misc::self_update()?;
        }

        Commands::Doctor { all, system, fix } => {
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

        Commands::Cleanup { yes } => {
            commands::misc::cleanup(yes, &config).await?;
        }

        Commands::SetDefault { version } => {
            commands::misc::set_default(version)?;
        }

        Commands::Lang { locale } => {
            commands::misc::set_language(locale)?;
        }

        Commands::InstallSelf { version, dir, with_self_update } => {
            commands::self_management::install_self(version, dir, with_self_update).await?;
        }

        Commands::UninstallSelf { dir, yes, purge, remove_config } => {
            commands::self_management::uninstall_self(dir, yes, purge, remove_config)?;
        }

        Commands::UpdateSelf { version, with_self_update } => {
            commands::self_management::update_self(version, with_self_update).await?;
        }

        Commands::Stats { json } => {
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
