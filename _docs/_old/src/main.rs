use std::env;

mod config;
mod i18n;
mod install;
mod utils;
mod versions;

fn main() -> anyhow::Result<()> {
    // Initialize i18n with NVM_LANG environment variable or default to English
    let nvm_lang = env::var("NVM_LANG").unwrap_or_else(|_| "en".to_string());
    let locale = if nvm_lang.starts_with("es") {
        i18n::Locale::Es
    } else {
        i18n::Locale::En
    };
    i18n::set_locale(locale);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_help();
        return Ok(());
    }

    let command = &args[1];
    let config = config::Config::new()?;

    match command.as_str() {
        "install" | "--install" | "-i" | "i" => {
            validate_command_args(&args, 3, "install <version>")?;
            let version = validate_and_normalize_version(&args[2])?;
            install::install_node(&config, &version)?;
        }
        "uninstall" | "--uninstall" | "-u" | "u" => {
            validate_command_args(&args, 3, "uninstall <version> [--force]")?;
            let version = validate_and_normalize_version(&args[2])?;
            let force = args.contains(&"--force".to_string());
            install::uninstall_node(&config, &version, force)?;
        }
        "use" | "--use" | "-U" => {
            validate_command_args(&args, 3, "use <version>")?;
            let version = validate_and_normalize_version(&args[2])?;
            versions::use_version(&config, &version)?;
        }
        "list" | "ls" | "--list" | "-l" => {
            versions::list_versions(&config)?;
        }
        "ls-remote" | "--ls-remote" | "-lr" => {
            versions::list_remote_versions(&config)?;
        }
        "current" | "--current" | "-c" => {
            versions::show_current_version(&config)?;
        }
        "help" | "--help" | "-h" => {
            show_help();
        }
        "version" | "--version" | "-v" => {
            println!("{}", t!("nvm_version", env!("CARGO_PKG_VERSION")));
        }
        "doctor" => {
            show_doctor_info(&config)?;
        }
        "lang" | "--lang" => {
            validate_command_args(&args, 3, "lang <locale>")?;
            let locale = &args[2];
            set_locale(locale)?;
        }
        _ => {
            println!("{}", t!("unknown_command", command));
            println!("{}", t!("run_help"));
        }
    }

    Ok(())
}

fn validate_command_args(args: &[String], min_len: usize, usage: &str) -> anyhow::Result<()> {
    if args.len() < min_len {
        anyhow::bail!("{}", t!("insufficient_arguments", usage, usage));
    }
    Ok(())
}

fn validate_and_normalize_version(version: &str) -> anyhow::Result<String> {
    if version.is_empty() {
        anyhow::bail!("{}", t!("version_empty"));
    }

    // Basic validation - should start with v or be numeric
    if version.starts_with('v') {
        if version.len() == 1 {
            anyhow::bail!("{}", t!("invalid_version_format"));
        }
        Ok(version.to_string())
    } else {
        // Check if it's a valid version number
        if version.chars().all(|c| c.is_numeric() || c == '.') {
            Ok(format!("v{}", version))
        } else {
            anyhow::bail!("{}", t!("invalid_version_format"));
        }
    }
}

fn show_doctor_info(config: &config::Config) -> anyhow::Result<()> {
    println!("{}", t!("doctor_title"));
    println!("{}", t!("doctor_separator"));
    println!(
        "{}",
        t!("doctor_directory", &config.nvm_dir.display().to_string())
    );
    println!();
    println!("{}", t!("doctor_installed_versions"));
    versions::list_versions(config)?;
    println!();
    println!("{}", t!("doctor_current_version"));
    versions::show_current_version(config)?;
    Ok(())
}

fn show_help() {
    println!("{}", t!("help_title"));
    println!("{}", t!("help_usage"));
    println!();
    println!("{}", t!("help_commands"));
    println!("{}", t!("help_install"));
    println!("{}", t!("help_uninstall"));
    println!("{}", t!("help_use"));
    println!("{}", t!("help_list"));
    println!("{}", t!("help_ls_remote"));
    println!("{}", t!("help_current"));
    println!("{}", t!("help_doctor"));
    println!("{}", t!("help_help"));
    println!("{}", t!("help_version"));
    println!("{}", t!("help_lang"));
}

fn set_locale(locale: &str) -> anyhow::Result<()> {
    match locale {
        "en" | "english" => {
            i18n::set_locale(i18n::Locale::En);
            println!("{}", t!("locale_set", "English"));
        }
        "es" | "spanish" => {
            i18n::set_locale(i18n::Locale::Es);
            println!("{}", t!("locale_set", "Español"));
        }
        _ => {
            return Err(anyhow::anyhow!("{}", t!("unsupported_locale", locale)));
        }
    }
    Ok(())
}
