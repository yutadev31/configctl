pub mod cli;
pub mod commands;
pub mod policy;

use std::{fs, path::Path};

use clap::Parser;
use serde::Deserialize;

use crate::{
    cli::{Cli, Command},
    commands::{apply::apply, check::check},
};

#[derive(Debug, Deserialize)]
struct ConfigctlToml {
    policy: String,
}

fn read_configctl_toml() -> ConfigctlToml {
    let path = "configctl.toml";
    let content =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    toml::from_str(&content).expect("Failed to parse configctl.toml")
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Check => {
            let config = read_configctl_toml();
            let policy_dir = Path::new(&config.policy);

            check(policy_dir);
        }
        Command::Apply => {
            let config = read_configctl_toml();
            let policy_dir = Path::new(&config.policy);

            apply(policy_dir);
        }
    }
}
