use std::{
    fs,
    os::unix,
    path::{Path, PathBuf},
};

use crate::configs::policy::PolicyToml;

fn apply_regular_file(policy_path: &Path, project_path: &Path) {
    let policy_content = fs::read_to_string(policy_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", policy_path.display(), e));

    let parent_dir = project_path.parent().unwrap();
    if parent_dir != Path::new("") && !parent_dir.is_dir() {
        fs::create_dir_all(parent_dir).unwrap_or_else(|e| {
            panic!("Failed to create directory {}: {}", parent_dir.display(), e)
        });

        println!("Created directory {}", parent_dir.display());
    } else if project_path.is_file() {
        let project_content = fs::read_to_string(project_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", project_path.display(), e));

        if project_content == policy_content {
            return;
        }
    }

    fs::write(project_path, policy_content)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", project_path.display(), e));
    println!("Wrote file {}", project_path.display());
}

fn apply_symlink(policy_path: &Path, project_path: &Path) {
    let policy_target = fs::read_link(policy_path)
        .unwrap_or_else(|e| panic!("Failed to read symlink {}: {}", policy_path.display(), e));

    if project_path.is_symlink() {
        let project_target = fs::read_link(project_path)
            .unwrap_or_else(|e| panic!("Failed to read symlink {}: {}", project_path.display(), e));

        if policy_target == project_target {
            return;
        } else {
            fs::remove_file(project_path).unwrap_or_else(|e| {
                panic!("Failed to remove symlink {}: {}", project_path.display(), e)
            });
            println!("Removed symlink {}", project_path.display());
        }
    } else if project_path.exists() {
        panic!(
            "Cannot create symlink at {}: path exists and is not a symlink",
            project_path.display()
        );
    }

    unix::fs::symlink(policy_target, project_path)
        .unwrap_or_else(|e| panic!("Failed to create symlink {}: {}", project_path.display(), e));
    println!("Created symlink {}", project_path.display());
}

fn apply_includes(policy_dir: &Path, includes: &[String]) {
    for file in includes {
        let policy_path = policy_dir.join("template").join(file);
        let project_path = PathBuf::from(file);

        if policy_path.is_symlink() {
            apply_symlink(&policy_path, &project_path);
        } else if policy_path.is_file() {
            apply_regular_file(&policy_path, &project_path);
        }
    }
}

fn apply_required(required: &[String]) {
    for file in required {
        let path = Path::new(file);
        if !path.is_file() {
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap_or_else(|e| panic!("Failed to create {}: {}", path.display(), e));
        }
    }
}

pub fn apply(policy_dir: &Path) {
    let policy = PolicyToml::from_file(policy_dir);
    if let Some(base) = policy.base {
        apply(&policy_dir.join(base));
    }

    apply_includes(policy_dir, &policy.includes);
    apply_required(&policy.required);
}
