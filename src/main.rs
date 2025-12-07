use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

mod config;
mod core;
mod i18n;
mod utils;

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

    /// Verify installation and system info
    Doctor,

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
}

#[tokio::main]
async fn main() -> Result<()> {
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
            use crate::core::{download, extract, versions, refresh_installed_cache};

            println!("{} {}", t!("installing_node"), version);

            // Obtener lista de versiones disponibles
            let available_versions = core::get_cached_versions(&config).await?;

            // Resolver la versión (puede ser un alias como "latest" o "lts")
            let resolved_version = versions::resolve_version(&version, &available_versions)?;

            // Buscar la versión completa en la lista
            let node_version = available_versions
                .iter()
                .find(|v| v.version == resolved_version)
                .ok_or_else(|| anyhow::anyhow!("Version {} not found", resolved_version))?;

            println!("Resolved to: {}", node_version.version);

            // Verificar si ya está instalada
            let version_dir = config.versions_dir().join(&node_version.version);
            if version_dir.exists() {
                println!("{}", t!("version_already_installed").replace("{}", &node_version.version));
                return Ok(());
            }

            // Crear directorio temporal para descargas
            let download_dir = config.nvm_dir.join("downloads");
            std::fs::create_dir_all(&download_dir)?;

            // Descargar archivo
            println!("\n{}", t!("downloading"));
            let archive_path = download::download_node_archive(node_version, &download_dir, &config).await?;

            // Crear directorio de versiones si no existe
            let versions_dir = config.versions_dir();
            std::fs::create_dir_all(&versions_dir)?;

            // Extraer archivo
            println!("\n{}", t!("extracting"));
            let extracted_path = extract::extract_archive(&archive_path, &versions_dir)?;

            // Mover al directorio final (versión normalizada)
            if extracted_path != version_dir {
                extract::move_extracted_files(&extracted_path, &version_dir)?;
            }

            // Actualizar cache de versiones instaladas
            refresh_installed_cache(&config)?;

            // Eliminar archivo de descarga
            std::fs::remove_file(&archive_path)?;

            println!("\n✓ {}", t!("installed_node").replace("{}", &node_version.version));
        }

        Commands::Uninstall { version, force } => {
            use crate::core::versions;

            println!("{}", t!("uninstalling_version").replace("{version}", &version));

            // Obtener versiones disponibles para resolver aliases
            let available_versions = core::get_cached_versions(&config).await?;

            // Resolver la versión (puede ser alias)
            let resolved_version = versions::resolve_version(&version, &available_versions)?;

            // Verificar que la versión esté instalada
            let version_dir = config.versions_dir().join(&resolved_version);
            if !version_dir.exists() {
                eprintln!("{}", t!("version_not_installed").replace("{}", &resolved_version));
                return Ok(());
            }

            // Verificar si es la versión actual
            if let Some(current_version) = versions::get_current_version(&config) {
                if current_version == resolved_version && !force {
                    eprintln!("{}", t!("cannot_uninstall_active")
                        .replace("{version}", &resolved_version));
                    eprintln!("{}", t!("use_force_to_uninstall"));
                    return Ok(());
                }

                // Si es la versión actual y se usa --force, eliminar el symlink
                if current_version == resolved_version && force {
                    let current_link = config.current_dir();
                    if current_link.exists() {
                        core::symlink::remove_symlink(&current_link)?;
                    }
                }
            }

            // Eliminar el directorio de la versión
            std::fs::remove_dir_all(&version_dir)
                .with_context(|| format!("Failed to remove version directory: {}", version_dir.display()))?;

            // Actualizar cache de versiones instaladas
            core::refresh_installed_cache(&config)?;

            println!("{}", t!("version_uninstalled").replace("{version}", &resolved_version));
        }

        Commands::Use { version } => {
            use crate::core::{symlink, versions};

            // Determinar qué versión usar
            let version_to_use = if let Some(ver) = version {
                ver
            } else {
                // Si no se especificó versión, buscar .nvmrc
                if let Some((nvmrc_path, nvmrc_version)) = versions::find_nvmrc_in_tree(None) {
                    println!("Found .nvmrc: {}", nvmrc_path.display());
                    println!("Using version from .nvmrc: {}", nvmrc_version);
                    nvmrc_version
                } else {
                    eprintln!("No version specified and no .nvmrc found");
                    eprintln!("Usage: nvm use <version>");
                    return Ok(());
                }
            };

            println!("Switching to Node.js {}...", version_to_use);

            // Obtener versiones disponibles para resolver aliases
            let available_versions = core::get_cached_versions(&config).await?;

            // Resolver la versión (puede ser alias)
            let resolved_version = versions::resolve_version(&version_to_use, &available_versions)?;

            // Verificar que la versión esté instalada
            let version_dir = config.versions_dir().join(&resolved_version);
            if !version_dir.exists() {
                eprintln!("{}", t!("version_not_installed").replace("{}", &resolved_version));
                eprintln!("Run: nvm install {}", resolved_version);
                return Ok(());
            }

            // Crear o actualizar symlink
            let current_link = config.current_dir();
            println!("{}", t!("creating_symlink"));

            // En Windows, el symlink apunta a la raíz de la versión
            // En Unix, debe apuntar a la subcarpeta bin/
            #[cfg(windows)]
            let symlink_target = &version_dir;

            #[cfg(not(windows))]
            let symlink_target = version_dir.join("bin");

            symlink::create_or_update_symlink(symlink_target, &current_link)
                .context("Failed to create symlink")?;

            // Persistir la versión en .nvm-version para recuperación confiable
            symlink::persist_current_version(&current_link, &resolved_version)
                .context("Failed to persist current version")?;

            println!("\n✓ {}", t!("now_using_node").replace("{}", &resolved_version));

            // Mostrar información adicional
            if symlink::is_valid_symlink(&current_link) {
                println!("{}", t!("symlink_created"));
            }
        }

        Commands::Ls => {
            use crate::core::versions;

            // Obtener versiones instaladas
            let mut installed = core::get_installed_versions(&config)?;

            if installed.is_empty() {
                println!("{}", t!("no_versions_installed"));
                return Ok(());
            }

            // Ordenar versiones
            versions::sort_versions(&mut installed);

            // Obtener versión actual
            let current = versions::get_current_version(&config);

            // Obtener información de versiones remotas para LTS info
            let available_versions = core::get_cached_versions(&config).await.unwrap_or_default();

            // Mostrar título
            println!("\n{}", t!("installed_versions"));

            // Mostrar cada versión
            for version in &installed {
                let is_current = current.as_ref().map(|c| c == version).unwrap_or(false);
                let formatted = versions::format_installed_version(version, is_current, &available_versions);
                println!("{}", formatted);
            }

            // Mostrar versión actual al final
            if let Some(curr) = current {
                println!("\n{} {}", t!("current_label"), curr);
            }
        }

        Commands::LsRemote { lts } => {
            use crate::core::versions::{format_version_display, VersionFilter};

            println!("{}", t!("fetching_versions"));

            // Get cached versions
            match core::get_cached_versions(&config).await {
                Ok(versions) => {
                    // Apply filters
                    let mut filter = VersionFilter::new();
                    if lts {
                        filter = filter.lts_only();
                    }

                    // Detect platform for filtering
                    #[cfg(target_os = "windows")]
                    let platform = "win";
                    #[cfg(not(target_os = "windows"))]
                    let platform = "linux";

                    filter = filter.platform(platform.to_string());

                    let filtered_versions = filter.apply(versions.clone());

                    // Limit display to prevent overwhelming output
                    let limit = 50;
                    let display_versions: Vec<_> = filtered_versions.iter().take(limit).collect();

                    // Show filter info
                    if lts {
                        println!("\n{}", t!("showing_lts_versions"));
                    }

                    println!(
                        "\n{} {} {} {}:",
                        t!("showing"),
                        display_versions.len(),
                        t!("of"),
                        filtered_versions.len()
                    );

                    // Display versions
                    for version in display_versions {
                        println!("{}", format_version_display(version));
                    }

                    // Show hint if there are more versions
                    if filtered_versions.len() > limit {
                        println!("\n{} {} {}",
                            t!("and"),
                            filtered_versions.len() - limit,
                            t!("more_versions")
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", t!("error_fetching_versions"), e);
                }
            }
        }

        Commands::Current => {
            use crate::core::versions;

            if let Some(current_version) = versions::get_current_version(&config) {
                println!("{}", current_version);
            } else {
                println!("{}", t!("no_current_version"));
            }
        }

        Commands::Alias { name, version } => {
            use crate::core::aliases::Aliases;
            use crate::core::versions;

            // Validar nombre de alias
            if !Aliases::is_valid_alias_name(&name) {
                eprintln!("{}", t!("invalid_alias_name").replace("{name}", &name));
                eprintln!("{}", t!("invalid_alias_name_help"));
                return Ok(());
            }

            // Obtener versiones disponibles para resolver aliases
            let available_versions = core::get_cached_versions(&config).await?;

            // Resolver la versión (puede ser otro alias o versión directa)
            let resolved_version = versions::resolve_version(&version, &available_versions)?;

            // Verificar que la versión exista (instalada o disponible)
            let version_dir = config.versions_dir().join(&resolved_version);
            if !version_dir.exists() {
                // Si no está instalada, verificar que al menos esté disponible
                let version_exists = available_versions.iter()
                    .any(|v| v.version == resolved_version);

                if !version_exists {
                    eprintln!("{}", t!("version_not_found").replace("{}", &resolved_version));
                    return Ok(());
                }
            }

            // Cargar aliases existentes
            let mut aliases = Aliases::load()?;

            // Verificar si es actualización o creación
            let is_update = aliases.contains(&name);

            // Establecer el alias
            aliases.set(name.clone(), resolved_version.clone());

            // Guardar aliases
            aliases.save()?;

            // Mostrar mensaje apropiado
            if is_update {
                println!("{}", t!("alias_updated")
                    .replace("{name}", &name)
                    .replace("{version}", &resolved_version));
            } else {
                println!("{}", t!("alias_created")
                    .replace("{name}", &name)
                    .replace("{version}", &resolved_version));
            }
        }

        Commands::Unalias { name } => {
            use crate::core::aliases::Aliases;

            // Cargar aliases existentes
            let mut aliases = Aliases::load()?;

            // Intentar eliminar el alias
            if let Some(removed_version) = aliases.remove(&name) {
                // Guardar cambios
                aliases.save()?;

                println!("{}", t!("alias_removed").replace("{name}", &name));
                println!("  {} -> {}", name, removed_version);
            } else {
                eprintln!("{}", t!("alias_not_found").replace("{name}", &name));
            }
        }

        Commands::Aliases => {
            use crate::core::aliases::Aliases;
            use colored::Colorize;

            // Cargar aliases
            let aliases = Aliases::load()?;

            // Verificar si hay aliases definidos
            if aliases.aliases.is_empty() {
                println!("{}", t!("no_aliases_defined"));
                return Ok(());
            }

            // Mostrar título
            println!("\n{}", t!("aliases_list_title"));

            // Obtener y mostrar aliases ordenados
            let list = aliases.list();
            for (name, version) in list {
                println!("  {} -> {}", name.cyan().bold(), version);
            }

            println!();
        }


        Commands::Doctor => {
            println!("Command 'doctor' - Basic implementation");
            show_doctor_info(&config)?;
        }

        Commands::Cleanup { yes } => {
            use crate::core::versions;
            use std::io::{self, Write};

            println!("{}", t!("cleaning_up"));

            // Obtener versiones instaladas
            let installed = core::get_installed_versions(&config)?;

            if installed.is_empty() {
                println!("{}", t!("no_versions_installed"));
                return Ok(());
            }

            // Obtener versión actual
            let current_version = versions::get_current_version(&config);

            // Obtener información de versiones remotas para identificar LTS
            let available_versions = core::get_cached_versions(&config).await.unwrap_or_default();

            // Determinar qué versiones mantener
            let mut versions_to_keep = Vec::new();
            let mut versions_to_remove = Vec::new();

            for version in &installed {
                let mut keep = false;

                // Mantener versión actual
                if let Some(ref current) = current_version {
                    if version == current {
                        keep = true;
                    }
                }

                // Mantener versiones LTS
                if let Some(node_version) = available_versions.iter().find(|v| &v.version == version) {
                    if node_version.lts.is_lts() {
                        keep = true;
                    }
                }

                if keep {
                    versions_to_keep.push(version.clone());
                } else {
                    versions_to_remove.push(version.clone());
                }
            }

            // Si no hay nada que eliminar
            if versions_to_remove.is_empty() {
                println!("{}", t!("no_versions_to_cleanup"));
                return Ok(());
            }

            // Mostrar información
            println!("\n{}", t!("cleanup_title"));
            for version in &versions_to_remove {
                println!("  - {}", version);
            }

            println!("\n{}", t!("cleanup_keeping"));
            if let Some(ref current) = current_version {
                println!("  {} ", t!("cleanup_current_version")
                    .replace("{version}", current));
            }
            let lts_count = versions_to_keep.iter()
                .filter(|v| {
                    available_versions.iter()
                        .find(|av| &av.version == *v)
                        .map(|av| av.lts.is_lts())
                        .unwrap_or(false)
                })
                .count();
            if lts_count > 0 {
                println!("  {}", t!("cleanup_lts_versions")
                    .replace("{count}", &lts_count.to_string()));
            }

            // Confirmación (si no se usa --yes)
            if !yes {
                print!("\n{}", t!("cleanup_confirm"));
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                let input = input.trim().to_lowercase();
                if input != "y" && input != "yes" && input != "s" && input != "si" {
                    println!("{}", t!("cleanup_cancelled"));
                    return Ok(());
                }
            }

            // Eliminar versiones
            let mut removed_count = 0;
            for version in &versions_to_remove {
                let version_dir = config.versions_dir().join(version);
                if version_dir.exists() {
                    std::fs::remove_dir_all(&version_dir)?;
                    removed_count += 1;
                }
            }

            // Actualizar cache
            core::refresh_installed_cache(&config)?;

            println!("\n{}", t!("cleanup_complete")
                .replace("{count}", &removed_count.to_string()));
        }

        #[cfg(feature = "self-update")]
        Commands::SelfUpdate => {
            use self_update::cargo_crate_version;

            println!("{}", t!("checking_for_updates"));

            let current_version = cargo_crate_version!();
            println!("{}", t!("current_version_label")
                .replace("{version}", current_version));

            // Configurar el actualizador
            let status = self_update::backends::github::Update::configure()
                .repo_owner("FreddyCamposeco")
                .repo_name("nvm-rs")
                .bin_name("nvm")
                .current_version(current_version)
                .show_download_progress(true)
                .no_confirm(false)
                .build()?
                .update()?;

            match status {
                self_update::Status::UpToDate(v) => {
                    println!("{}", t!("already_latest"));
                    println!("Version: {}", v);
                }
                self_update::Status::Updated(v) => {
                    println!("{}", t!("update_complete"));
                    println!("{}", t!("updating_to").replace("{version}", &v));
                    println!("\n{}", t!("restart_required"));
                }
            }
        }

        Commands::SetDefault { version } => {
            println!("Command 'set-default {}' - Not yet implemented", version);
            println!("This will be implemented in Phase 7 of the migration plan");
        }

        Commands::Lang { locale } => {
            if let Some(new_locale) = Locale::from_str(&locale) {
                set_locale(new_locale);
                println!("{}", t!("locale_set", new_locale.as_str()));
            } else {
                println!("{}", t!("unsupported_locale", &locale));
            }
        }

        Commands::InstallSelf { version, dir, with_self_update } => {
            use core::installer::*;

            println!("{}", t!("install_self_start"));

            // Determinar versión a instalar
            let release = if let Some(ver) = version {
                if ver == "latest" {
                    get_latest_release().await?
                } else {
                    get_release_by_tag(&ver).await?
                }
            } else {
                get_latest_release().await?
            };

            println!("{}", t!("install_self_version")
                .replace("{version}", &release.tag_name));

            // Determinar asset apropiado
            let asset_name = get_platform_asset_name(&release.tag_name, with_self_update);
            let asset = release.assets.iter()
                .find(|a| a.name == asset_name)
                .ok_or_else(|| anyhow::anyhow!(
                    "{}", t!("install_self_no_asset")
                        .replace("{asset}", &asset_name)
                ))?;

            // Crear directorio temporal
            let temp_dir = std::env::temp_dir().join("nvm-install");
            std::fs::create_dir_all(&temp_dir)?;
            let download_path = temp_dir.join(&asset.name);

            // Descargar binario
            println!("\n{}", t!("downloading"));
            download_asset(asset, &download_path).await?;

            // Verificar checksum (si está disponible)
            println!("{}", t!("install_self_verifying"));
            let checksum = verify_checksum(&download_path, None).await?;
            println!("SHA256: {}", checksum);

            // Determinar directorio de instalación
            let install_dir = if let Some(d) = dir {
                d
            } else {
                get_install_dir()?
            };

            // Instalar binario
            println!("\n{}", t!("install_self_installing"));
            let installed_path = install_binary(&download_path, &install_dir)?;

            // Limpiar archivos temporales
            let _ = std::fs::remove_dir_all(&temp_dir);

            println!("\n✓ {}", t!("install_self_complete")
                .replace("{path}", &installed_path.display().to_string()));

            // Configurar variables de entorno
            #[cfg(windows)]
            {
                use core::installer::{add_to_path, set_nvm_home, is_in_path};

                println!("\n{}", t!("install_self_configuring_env"));

                // Configurar NVM_HOME
                let nvm_data_dir = dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".nvm");

                if let Err(e) = set_nvm_home(&nvm_data_dir) {
                    println!("{}", t!("install_self_env_warning")
                        .replace("{error}", &e.to_string()));
                } else {
                    println!("✓ {}", t!("install_self_nvm_dir_set")
                        .replace("{path}", &nvm_data_dir.display().to_string()));
                }

                // Agregar nvm/bin al PATH si no está
                if !is_in_path(&install_dir) {
                    if let Err(e) = add_to_path(&install_dir) {
                        println!("{}", t!("install_self_path_warning")
                            .replace("{error}", &e.to_string()));
                        println!("\n{}", get_path_instructions(&install_dir));
                    } else {
                        println!("✓ {}", t!("install_self_path_set"));
                    }
                } else {
                    println!("✓ {}", t!("install_self_path_already_set"));
                }

                // Agregar nvm/current/bin al PATH para la versión activa
                let current_dir = nvm_data_dir.join("current").join("bin");
                if !is_in_path(&current_dir) {
                    if let Err(e) = add_to_path(&current_dir) {
                        println!("{}", t!("install_self_path_warning")
                            .replace("{error}", &e.to_string()));
                    } else {
                        println!("✓ PATH configurado para versión activa ({})", current_dir.display());
                    }
                } else {
                    println!("✓ PATH de versión activa ya configurado");
                }

                println!("\n{}", t!("install_self_restart_terminal"));
            }

            // Verificar si está en PATH
            #[cfg(not(windows))]
            if !is_in_path(&install_dir) {
                println!("\n⚠ {}", t!("install_self_not_in_path"));
                println!("\n{}", get_path_instructions(&install_dir));
            }
        }

        Commands::UninstallSelf { dir, yes } => {
            use core::installer::*;
            use std::io::{self, Write};

            println!("{}", t!("uninstall_self_start"));

            // Determinar directorio
            let install_dir = if let Some(d) = dir {
                d
            } else {
                get_install_dir()?
            };

            #[cfg(windows)]
            let exe_path = install_dir.join("nvm.exe");
            #[cfg(not(windows))]
            let exe_path = install_dir.join("nvm");

            if !exe_path.exists() {
                println!("{}", t!("uninstall_self_not_found")
                    .replace("{path}", &exe_path.display().to_string()));
                return Ok(());
            }

            // Confirmación
            if !yes {
                print!("\n{} ", t!("uninstall_self_confirm")
                    .replace("{path}", &exe_path.display().to_string()));
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                let input = input.trim().to_lowercase();
                if input != "y" && input != "yes" && input != "s" && input != "si" {
                    println!("{}", t!("uninstall_self_cancelled"));
                    return Ok(());
                }
            }

            // Usar limpieza completa
            let data_dir = dirs::home_dir()
                .map(|h| h.join(".nvm"));

            full_uninstall_cleanup(Some(&install_dir), data_dir.as_deref())?;
        }

        Commands::UpdateSelf { version, with_self_update } => {
            use core::installer::*;

            println!("{}", t!("update_self_start"));

            // Obtener ejecutable actual
            let current_exe = get_current_executable()?;
            let install_dir = current_exe.parent()
                .ok_or_else(|| anyhow::anyhow!("Cannot determine installation directory"))?;

            // Determinar versión a instalar
            let release = if let Some(ver) = version {
                if ver == "latest" {
                    get_latest_release().await?
                } else {
                    get_release_by_tag(&ver).await?
                }
            } else {
                get_latest_release().await?
            };

            println!("{}", t!("update_self_version")
                .replace("{version}", &release.tag_name));

            // Determinar asset apropiado
            let asset_name = get_platform_asset_name(&release.tag_name, with_self_update);
            let asset = release.assets.iter()
                .find(|a| a.name == asset_name)
                .ok_or_else(|| anyhow::anyhow!(
                    "{}", t!("install_self_no_asset")
                        .replace("{asset}", &asset_name)
                ))?;

            // Crear directorio temporal
            let temp_dir = std::env::temp_dir().join("nvm-update");
            std::fs::create_dir_all(&temp_dir)?;
            let download_path = temp_dir.join(&asset.name);

            // Descargar binario
            println!("\n{}", t!("downloading"));
            download_asset(asset, &download_path).await?;

            // Verificar checksum
            println!("{}", t!("install_self_verifying"));
            let checksum = verify_checksum(&download_path, None).await?;
            println!("SHA256: {}", checksum);

            // Actualizar binario
            println!("\n{}", t!("update_self_installing"));
            let _installed_path = install_binary(&download_path, install_dir)?;

            // Limpiar archivos temporales
            let _ = std::fs::remove_dir_all(&temp_dir);

            println!("\n✓ {}", t!("update_self_complete")
                .replace("{version}", &release.tag_name));
            println!("\n{}", t!("restart_required"));
        }
    }

    Ok(())
}

fn show_doctor_info(config: &Config) -> Result<()> {
    use utils::{print_check, print_success, print_warning, print_x};

    println!("\n{}", t!("doctor_title"));
    println!("{}", "=".repeat(50));

    // Check NVM directory
    if config.nvm_dir.exists() {
        print_check();
        println!(
            "{}: {}",
            t!("doctor_directory", ""),
            config.nvm_dir.display()
        );
    } else {
        print_x();
        println!("NVM directory does not exist");
    }

    // Check installed versions
    let installed = core::get_installed_versions(config)?;
    print_check();
    println!("{}: {}", t!("doctor_installed_versions"), installed.len());

    // Check connectivity (simple test)
    print!("{} ", t!("doctor_connectivity"));
    // Use blocking client to avoid async issues
    let handle = std::thread::spawn(|| {
        reqwest::blocking::get("https://nodejs.org")
    });

    match handle.join() {
        Ok(Ok(_)) => print_success("OK"),
        _ => print_warning("Failed"),
    }

    // Check symlink support
    print!("{} ", t!("doctor_symlink_support"));
    #[cfg(unix)]
    print_success("Supported");

    #[cfg(windows)]
    {
        // On Windows, it depends on permissions
        print_warning("Check required (admin rights may be needed)");
    }

    println!();
    Ok(())
}
