use std::{path::PathBuf, process};

pub fn clone(repo: &str) -> PathBuf {
    let repo_name = repo.split('/').next_back().unwrap();
    let repo_dir = dirs::state_dir()
        .expect("Failed to get state directory")
        .join("configctl")
        .join(repo_name);

    if repo_dir.exists() {
        println!("Running git pull...");
        process::Command::new("git")
            .arg("pull")
            .current_dir(&repo_dir)
            .status()
            .expect("Failed to clone repository");
        println!("Success git pull");
    } else {
        println!("Running git clone...");
        process::Command::new("git")
            .arg("clone")
            .arg(repo)
            .arg(&repo_dir)
            .status()
            .expect("Failed to clone repository");
        println!("Success git clone");
    }

    repo_dir
}
