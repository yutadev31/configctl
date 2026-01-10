use std::{fs, path::Path, process};

use serde::Deserialize;

const POLICY_FILENAME: &str = "policy.toml";

#[derive(Debug, Deserialize)]
pub struct PolicyToml {
    pub base: Option<String>,
    pub includes: Vec<String>,
    pub required: Vec<String>,
}

impl PolicyToml {
    pub fn from_file(dir: &Path) -> Self {
        let toml = fs::read_to_string(dir.join(POLICY_FILENAME)).unwrap_or_else(|e| {
            eprintln!("Failed to read {}: {}", POLICY_FILENAME, e);
            process::exit(1);
        });

        toml::from_str(&toml).expect(&format!("Failed to parse {POLICY_FILENAME}"))
    }
}
