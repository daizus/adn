use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug, Deserialize)]
pub struct Bridge {
    pub interfaces: Vec<String>,
    pub ip: Option<String>,
    pub dhcp: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Vlan {
    pub id: u16,
    pub parent: String,
    pub ip: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bridge: Option<std::collections::HashMap<String, Bridge>>,
    pub vlan: Option<std::collections::HashMap<String, Vlan>>,
}

pub fn resolve_config_path(input: &Option<PathBuf>) -> PathBuf {
    match input {
        Some(path) => path.clone(),
        None => Path::new("/etc/netcraft.toml").to_path_buf(),
    }
}

pub fn load_config(path: &Path) -> Config {
    if !path.exists() {
        eprintln!("⚠️  No configuration file found at {}", path.display());
        eprintln!("   You can create one manually or run `netcraft --init` (soon!)");
        process::exit(1);
    }

    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("❌ Failed to read config file: {}", e);
        process::exit(1);
    });

    toml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("❌ Failed to parse TOML config: {}", e);
        process::exit(1);
    })
}
