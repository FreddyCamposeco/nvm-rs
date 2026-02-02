use std::env;
use std::path::Path;

pub fn is_in_path(install_dir: &Path) -> bool {
    if let Ok(path_var) = env::var("PATH") {
        let install_dir_str = install_dir.to_string_lossy();
        return path_var
            .split(if cfg!(windows) { ';' } else { ':' })
            .any(|p| p == install_dir_str);
    }
    false
}
