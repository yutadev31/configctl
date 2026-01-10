pub mod cli;
pub mod commands;
pub mod configs;
pub mod policy_repository;

use clap::Parser;

use crate::{
    cli::{Cli, Command},
    commands::{apply::apply, check::check, init::init},
    configs::configctl_toml::Config,
    policy_repository::clone,
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { repository, path } => {
            init(repository, path);
            println!("Successfully initialized configctl");
        }
        Command::Check => {
            let config = Config::from_file();
            let policy_dir = clone(&config.policy.repository).join(config.policy.path);

            check(&policy_dir);
            println!("Successfully checked policy");
        }
        Command::Apply => {
            let config = Config::from_file();
            let policy_dir = clone(&config.policy.repository).join(config.policy.path);

            apply(&policy_dir);
            println!("Successfully applied policy");
        }
    }
}
