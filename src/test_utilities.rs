#![cfg(test)]

use std::{env, fs, path::PathBuf};
use uuid::Uuid;

pub struct TestWorkspace {
    pub directory: PathBuf,
    git_twin_directory: PathBuf,
    original_current_dir: PathBuf,
}

impl TestWorkspace {
    pub fn setup() -> TestWorkspace {
        let directory = path_to_temporary_workspace();
        let git_twin_directory = path_to_temporary_workspace();
        fs::create_dir_all(&directory).expect("Couldn't create temporary workspace directory");
        fs::create_dir_all(&git_twin_directory)
            .expect("Couldn't create temporary git twin directory");
        let original_current_dir = env::current_dir().unwrap();
        env::set_current_dir(&directory).expect("Couldn't set CWD to temp workspace");
        TestWorkspace {
            directory,
            original_current_dir,
            git_twin_directory,
        }
    }

    pub fn teardown(&self) {
        env::set_current_dir(&self.original_current_dir)
            .expect("Couldn't reset CWD in workspace teardown");
        fs::remove_dir_all(&self.directory).expect("Couldn't delete the repository directory");
        fs::remove_dir_all(&self.git_twin_directory)
            .expect("Couldn't delete the repository directory");
    }
}

fn path_to_temporary_workspace() -> PathBuf {
    let directory_name = format!("temporary_workspace_{}", Uuid::new_v4());
    env::temp_dir().join(directory_name)
}
