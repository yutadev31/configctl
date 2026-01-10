use std::{fs, process};

use serde::{Deserialize, Serialize};

const CONFIG_FILENAME: &str = "configctl.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub policy: Policy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Policy {
    pub repository: String,
    pub path: String,
}

impl Config {
    pub fn from_file() -> Self {
        let toml = fs::read_to_string(CONFIG_FILENAME).unwrap_or_else(|e| {
            eprintln!("Failed to read {}: {}", CONFIG_FILENAME, e);
            process::exit(1);
        });

        toml::from_str(&toml).expect(&format!("Failed to parse {CONFIG_FILENAME}"))
    }
}
