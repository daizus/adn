use std::path::{Path, PathBuf};

pub fn resolve_config_path(input: &Option<PathBuf>) -> PathBuf {
    match input {
        Some(path) => path.clone(),
        None => Path::new("/etc/netcraft.toml").to_path_buf(),
    }
}
