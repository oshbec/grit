#![cfg(test)]

use std::{env, fs, path::PathBuf};
use uuid::Uuid;

pub struct TempWorkspace {
    pub directory: PathBuf,
    original_working_directory: PathBuf,
}

impl TempWorkspace {
    pub fn setup() -> TempWorkspace {
        let workspace_path = path_to_temporary_workspace();
        fs::create_dir_all(&workspace_path).expect("Couldn't create temporary workspace directory");
        let original_working_directory = env::current_dir().unwrap();
        env::set_current_dir(&workspace_path).expect("Couldn't set CWD to temp workspace");
        TempWorkspace {
            directory: workspace_path,
            original_working_directory,
        }
    }

    pub fn teardown(&self) {
        env::set_current_dir(&self.original_working_directory)
            .expect("Couldn't reset CWD in workspace teardown");
        fs::remove_dir_all(&self.directory).expect("Couldn't delete the repository directory");
    }
}

pub fn path_to_temporary_workspace() -> PathBuf {
    let directory_name = format!("temporary_workspace_{}", Uuid::new_v4());
    env::temp_dir().join(directory_name)
}
