use std::env;
use std::path::PathBuf;

pub fn get_path_instructions() -> String {
    let shell_config = if PathBuf::from(env::var("HOME").unwrap_or_default())
        .join(".zshrc")
        .exists()
    {
        "~/.zshrc"
    } else {
        "~/.bashrc"
    };

    let nvm_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".nvm");

    format!(
        r#"Para agregar nvm al PATH permanentemente:
1. Agregar al final de {}:
   export NVM_HOME=\"{}\"
   export NVM_BIN=\"$NVM_HOME/bin\"
   export NVM_NODE=\"$NVM_HOME/current/bin\"
   export PATH=\"$NVM_BIN:$NVM_NODE:$PATH\"
2. Recargar la configuración: source {}"#,
        shell_config,
        nvm_dir.display(),
        shell_config
    )
}
