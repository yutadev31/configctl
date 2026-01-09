use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Check,
    Apply,
}

#[derive(Debug, Deserialize)]
struct PolicyToml {
    includes: Vec<String>,
    required: Vec<String>,
}

fn read_policy(policy_dir: &str) -> PolicyToml {
    let toml = fs::read_to_string(format!("{policy_dir}/policy.toml"))
        .expect("Failed to read policy.toml");
    toml::from_str::<PolicyToml>(&toml).unwrap()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Check => {
            let policy_dir = "../config-policy";
            let mut is_failed = false;
            let policy = read_policy(policy_dir);

            for file in policy.includes {
                let policy_file_path = PathBuf::from(format!("{policy_dir}/template/{file}"));
                if policy_file_path.is_file() {
                    let policy_content = fs::read_to_string(policy_file_path).unwrap_or_else(|e| {
                        panic!("Failed to read policy template `{}`: {}", file, e)
                    });

                    let project_file_path = format!("{file}");
                    if fs::exists(&project_file_path).unwrap() {
                        let project_content = fs::read_to_string(&project_file_path).unwrap();

                        if policy_content == project_content {
                            println!("OK: `{}`", project_file_path);
                        } else {
                            is_failed = true;
                            eprintln!(
                                "Mismatch: `{}` does not match the policy template",
                                project_file_path
                            );
                        }
                    } else {
                        is_failed = true;
                        eprintln!(
                            "Missing file: `{}` does not exist in the project",
                            project_file_path
                        );
                    }
                } else if policy_file_path.is_symlink() {
                    let project_file_path = PathBuf::from(file);

                    if !project_file_path.exists() {
                        is_failed = true;
                        eprintln!(
                            "Missing file: `{}` does not exist in the project",
                            project_file_path.display()
                        );
                        continue;
                    }

                    if !project_file_path.is_symlink() {
                        is_failed = true;
                        eprintln!(
                            "Type mismatch: `{}` should be a symbolic link",
                            project_file_path.display()
                        );
                        continue;
                    }

                    let policy_target = fs::read_link(&policy_file_path).unwrap_or_else(|e| {
                        panic!(
                            "Failed to read symbolic link target `{}`: {}",
                            policy_file_path.display(),
                            e
                        )
                    });

                    let project_target = fs::read_link(&project_file_path).unwrap_or_else(|e| {
                        panic!(
                            "Failed to read symbolic link target `{}`: {}",
                            project_file_path.display(),
                            e
                        )
                    });

                    if policy_target == project_target {
                        println!("OK: `{}` (symbolic link)", project_file_path.display());
                    } else {
                        is_failed = true;
                        eprintln!(
                            "Mismatch: symbolic link `{}` points to `{}`, expected `{}`",
                            project_file_path.display(),
                            project_target.display(),
                            policy_target.display()
                        );
                    }
                }
            }

            for file in policy.required {
                let project_file_path = format!("{file}");
                if Path::new(&project_file_path).is_file() {
                    println!("OK: `{}` (required file)", project_file_path);
                } else {
                    is_failed = true;
                    eprintln!(
                        "Missing file: `{}` does not exist in the project",
                        project_file_path
                    );
                }
            }

            if is_failed {
                exit(1);
            }
        }
        Command::Apply => {}
    }
}
