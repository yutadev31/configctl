pub mod cli;
pub mod commands;
pub mod configs;
pub mod policy_repository;

use clap::Parser;

use crate::{
    cli::{Cli, Command},
    commands::{apply::apply, check::check},
    configs::configctl_toml::read_configctl_toml,
    policy_repository::clone,
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Check => {
            let config = read_configctl_toml();
            let policy_dir = clone(&config.policy.repository).join(config.policy.path);

            check(&policy_dir);
            println!("Successfully checked policy");
        }
        Command::Apply => {
            let config = read_configctl_toml();
            let policy_dir = clone(&config.policy.repository).join(config.policy.path);

            apply(&policy_dir);
            println!("Successfully applied policy");
        }
    }
}
