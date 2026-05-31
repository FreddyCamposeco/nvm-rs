use anyhow::{bail, Result};

pub fn shell_init(shell: &str) -> Result<()> {
    let script = match shell {
        "bash" => BASH_SCRIPT,
        "zsh" => ZSH_SCRIPT,
        "fish" => FISH_SCRIPT,
        "powershell" | "pwsh" => POWERSHELL_SCRIPT,
        other => bail!(
            "Unsupported shell: '{}'. Supported: bash, zsh, fish, powershell",
            other
        ),
    };
    print!("{}", script);
    Ok(())
}

// Auto-use .nvmrc on directory change (bash)
// Add to ~/.bashrc:  eval "$(nvm shell-init bash)"
const BASH_SCRIPT: &str = r#"
_nvm_auto_use() {
  local nvmrc
  if [ -f ".nvmrc" ]; then
    nvmrc="$(cat .nvmrc | tr -d '[:space:]')"
    if [ -n "$nvmrc" ]; then
      nvm use "$nvmrc" --silent 2>/dev/null || true
    fi
  fi
}
if [[ "$PROMPT_COMMAND" != *"_nvm_auto_use"* ]]; then
  PROMPT_COMMAND="_nvm_auto_use${PROMPT_COMMAND:+; $PROMPT_COMMAND}"
fi
"#;

// Auto-use .nvmrc on directory change (zsh)
// Add to ~/.zshrc:  eval "$(nvm shell-init zsh)"
const ZSH_SCRIPT: &str = r#"
autoload -U add-zsh-hook
_nvm_auto_use() {
  if [ -f ".nvmrc" ]; then
    local nvmrc
    nvmrc="$(cat .nvmrc | tr -d '[:space:]')"
    [ -n "$nvmrc" ] && nvm use "$nvmrc" --silent 2>/dev/null || true
  fi
}
add-zsh-hook chpwd _nvm_auto_use
_nvm_auto_use
"#;

// Auto-use .nvmrc on directory change (fish)
// Add to ~/.config/fish/config.fish:  nvm shell-init fish | source
const FISH_SCRIPT: &str = r#"
function _nvm_auto_use --on-variable PWD
  if test -f .nvmrc
    set -l nvmrc (string trim (cat .nvmrc))
    if test -n "$nvmrc"
      nvm use $nvmrc --silent 2>/dev/null
    end
  end
end
_nvm_auto_use
"#;

// Auto-use .nvmrc on directory change (PowerShell)
// Add to $PROFILE:  Invoke-Expression (nvm shell-init powershell)
const POWERSHELL_SCRIPT: &str = r#"
function global:_nvm_auto_use {
  if (Test-Path .nvmrc) {
    $nvmrc = (Get-Content .nvmrc -Raw).Trim()
    if ($nvmrc) {
      nvm use $nvmrc --silent 2>$null
    }
  }
}
function global:Set-Location {
  param([Parameter(ValueFromRemainingArguments)]$args)
  Microsoft.PowerShell.Management\Set-Location @args
  _nvm_auto_use
}
_nvm_auto_use
"#;
