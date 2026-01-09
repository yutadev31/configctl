use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub policy: Policy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Policy {
    pub repository: String,
    pub path: String,
}

pub fn read_configctl_toml() -> Config {
    let path = "configctl.toml";
    let content =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    toml::from_str(&content).expect("Failed to parse configctl.toml")
}
