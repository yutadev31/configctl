use std::{
    fs::{self, OpenOptions},
    os::unix,
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

fn read_policy(policy_dir: &Path) -> PolicyToml {
    let path = policy_dir.join("policy.toml");
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

    toml::from_str(&content).expect("Failed to parse policy.toml")
}

fn check_regular_file(policy_path: &Path, project_path: &Path) -> bool {
    let policy_content = fs::read_to_string(policy_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", policy_path.display(), e));

    if !project_path.exists() {
        eprintln!(
            "Missing file: `{}` does not exist in the project",
            project_path.display()
        );
        return false;
    }

    let project_content = fs::read_to_string(project_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", project_path.display(), e));

    if policy_content == project_content {
        println!("OK: `{}`", project_path.display());
        true
    } else {
        eprintln!(
            "Mismatch: `{}` does not match the policy template",
            project_path.display()
        );
        false
    }
}

fn check_symlink(policy_path: &Path, project_path: &Path) -> bool {
    if !project_path.exists() {
        eprintln!(
            "Missing file: `{}` does not exist in the project",
            project_path.display()
        );
        return false;
    }

    if !project_path.is_symlink() {
        eprintln!(
            "Type mismatch: `{}` should be a symbolic link",
            project_path.display()
        );
        return false;
    }

    let policy_target = fs::read_link(policy_path)
        .unwrap_or_else(|e| panic!("Failed to read symlink {}: {}", policy_path.display(), e));

    let project_target = fs::read_link(project_path)
        .unwrap_or_else(|e| panic!("Failed to read symlink {}: {}", project_path.display(), e));

    if policy_target == project_target {
        println!("OK: `{}` (symbolic link)", project_path.display());
        true
    } else {
        eprintln!(
            "Mismatch: symbolic link `{}` points to `{}`, expected `{}`",
            project_path.display(),
            project_target.display(),
            policy_target.display()
        );
        false
    }
}

fn check_includes(policy_dir: &Path, includes: &[String]) -> bool {
    let mut failed = false;

    for file in includes {
        let policy_path = policy_dir.join("template").join(file);
        let project_path = PathBuf::from(file);

        let ok = if policy_path.is_file() {
            check_regular_file(&policy_path, &project_path)
        } else if policy_path.is_symlink() {
            check_symlink(&policy_path, &project_path)
        } else {
            eprintln!(
                "Invalid policy entry: `{}` is neither file nor symlink",
                policy_path.display()
            );
            false
        };

        failed |= !ok;
    }

    !failed
}

fn check_required(required: &[String]) -> bool {
    let mut failed = false;

    for file in required {
        let path = Path::new(file);
        if path.is_file() {
            println!("OK: `{}` (required file)", path.display());
        } else {
            eprintln!(
                "Missing file: `{}` does not exist in the project",
                path.display()
            );
            failed = true;
        }
    }

    !failed
}

fn apply_regular_file(policy_path: &Path, project_path: &Path) {
    let policy_content = fs::read_to_string(policy_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", policy_path.display(), e));

    let parent_dir = project_path.parent().unwrap();
    fs::create_dir_all(&parent_dir)
        .unwrap_or_else(|e| panic!("Failed to create directory {}: {}", parent_dir.display(), e));

    fs::write(project_path, policy_content)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", project_path.display(), e));
}

fn apply_symlink(policy_path: &Path, project_path: &Path) {
    let policy_target = fs::read_link(policy_path)
        .unwrap_or_else(|e| panic!("Failed to read symlink {}: {}", policy_path.display(), e));

    unix::fs::symlink(policy_target, project_path)
        .unwrap_or_else(|e| panic!("Failed to create symlink {}: {}", project_path.display(), e));
}

fn apply_includes(policy_dir: &Path, includes: &[String]) {
    for file in includes {
        let policy_path = policy_dir.join("template").join(file);
        let project_path = PathBuf::from(file);

        if policy_path.is_file() {
            apply_regular_file(&policy_path, &project_path);
        } else if policy_path.is_symlink() {
            apply_symlink(&policy_path, &project_path);
        }
    }
}

fn apply_required(required: &[String]) {
    for file in required {
        let path = Path::new(file);
        if !path.is_file() {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(&path)
                .unwrap_or_else(|e| panic!("Failed to create {}: {}", path.display(), e));
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Check => {
            let policy_dir = Path::new("../config-policy");
            let policy = read_policy(policy_dir);

            let includes_ok = check_includes(policy_dir, &policy.includes);
            let required_ok = check_required(&policy.required);

            if !(includes_ok && required_ok) {
                exit(1);
            }
        }
        Command::Apply => {
            let policy_dir = Path::new("../config-policy");
            let policy = read_policy(policy_dir);

            apply_includes(policy_dir, &policy.includes);
            apply_required(&policy.required);
        }
    }
}
