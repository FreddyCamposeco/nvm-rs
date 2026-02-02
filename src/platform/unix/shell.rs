use crate::error::{message, with_context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
pub(crate) enum ShellType {
    Bash,
    Zsh,
    Fish,
}

const NVM_BLOCK_START: &str = "# >>> nvm-rs >>>";
const NVM_BLOCK_END: &str = "# <<< nvm-rs <<<";

pub(crate) fn detect_shell_config() -> Result<(PathBuf, ShellType)> {
    let home = dirs::home_dir().ok_or_else(|| message("Home directory not found"))?;
    let shell = env::var("SHELL").unwrap_or_default();

    if shell.contains("fish") {
        return Ok((home.join(".config").join("fish").join("config.fish"), ShellType::Fish));
    }

    if shell.contains("zsh") {
        return Ok((home.join(".zshrc"), ShellType::Zsh));
    }

    if shell.contains("bash") {
        let bashrc = home.join(".bashrc");
        let bash_profile = home.join(".bash_profile");
        if bashrc.exists() {
            return Ok((bashrc, ShellType::Bash));
        }
        if bash_profile.exists() {
            return Ok((bash_profile, ShellType::Bash));
        }
        return Ok((bashrc, ShellType::Bash));
    }

    let zshrc = home.join(".zshrc");
    if zshrc.exists() {
        return Ok((zshrc, ShellType::Zsh));
    }

    Ok((home.join(".bashrc"), ShellType::Bash))
}

pub(crate) fn build_shell_block(nvm_dir: &Path, shell: ShellType) -> String {
    let nvm_dir_str = nvm_dir.to_string_lossy();

    match shell {
        ShellType::Fish => format!(
            "{start}\nset -gx NVM_HOME \"{nvm_dir}\"\nset -gx NVM_BIN \"$NVM_HOME/bin\"\nset -gx NVM_NODE \"$NVM_HOME/current/bin\"\nfish_add_path $NVM_BIN $NVM_NODE\n{end}\n",
            start = NVM_BLOCK_START,
            end = NVM_BLOCK_END,
            nvm_dir = nvm_dir_str
        ),
        ShellType::Bash | ShellType::Zsh => format!(
            "{start}\nexport NVM_HOME=\"{nvm_dir}\"\nexport NVM_BIN=\"$NVM_HOME/bin\"\nexport NVM_NODE=\"$NVM_HOME/current/bin\"\nexport PATH=\"$NVM_BIN:$NVM_NODE:$PATH\"\n{end}\n",
            start = NVM_BLOCK_START,
            end = NVM_BLOCK_END,
            nvm_dir = nvm_dir_str
        ),
    }
}

pub(crate) fn ensure_shell_block(config_path: &Path, block: &str) -> Result<()> {
    let mut contents = if config_path.exists() {
        fs::read_to_string(config_path).unwrap_or_default()
    } else {
        String::new()
    };

    if contents.contains(NVM_BLOCK_START) && contents.contains(NVM_BLOCK_END) {
        return Ok(());
    }

    if !contents.ends_with('\n') && !contents.is_empty() {
        contents.push('\n');
    }
    contents.push_str(block);

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(config_path, contents)
        .map_err(|e| with_context("Failed to update shell configuration", e))?;
    Ok(())
}

pub(crate) fn remove_shell_block(config_path: &Path) -> Result<()> {
    if !config_path.exists() {
        return Ok(());
    }

    let contents = fs::read_to_string(config_path).unwrap_or_default();
    let mut output = String::new();
    let mut skipping = false;

    for line in contents.lines() {
        if line.trim() == NVM_BLOCK_START {
            skipping = true;
            continue;
        }
        if line.trim() == NVM_BLOCK_END {
            skipping = false;
            continue;
        }
        if !skipping {
            output.push_str(line);
            output.push('\n');
        }
    }

    fs::write(config_path, output)
        .map_err(|e| with_context("Failed to update shell configuration", e))?;
    Ok(())
}
