use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PolicyToml {
    pub base: Option<String>,
    pub includes: Vec<String>,
    pub required: Vec<String>,
}

pub fn read_policy(policy_dir: &Path) -> PolicyToml {
    let path = policy_dir.join("policy.toml");
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

    toml::from_str(&content).expect("Failed to parse policy.toml")
}
