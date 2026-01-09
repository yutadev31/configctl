use std::{fs, path::Path, process::exit};

use crate::configs::configctl_toml::{Config, Policy};

pub fn init(repository: String, path: String) {
    let toml_path = Path::new("configctl.toml");
    if toml_path.is_file() {
        eprintln!("configctl.toml already exists");
        exit(1);
    }

    let config = Config {
        policy: Policy {
            repository,
            path: path.clone(),
        },
    };

    let content = toml::to_string(&config).unwrap();
    fs::write(toml_path, content).expect("Failed to write configctl.toml");
}
