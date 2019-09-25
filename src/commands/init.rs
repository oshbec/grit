use std::{env, fs, path::PathBuf};

pub fn run(directory: Option<&PathBuf>) {
    let current_dir = env::current_dir().expect("Couldn't identify current working directory");
    let directory = directory.unwrap_or(&current_dir);
    let required_directories = vec![".git", ".git/objects", ".git/refs"];
    for required_directory in required_directories {
        let required_directory = directory.join(required_directory);
        fs::create_dir_all(required_directory).expect("Could not create required directory");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_utilities::TempWorkspace;

    #[test]
    fn initializes_repository_in_existing_directory() {
        let workspace = TempWorkspace::setup();
        assert_eq!(
            workspace.directory.is_dir(),
            true,
            "The workspace directory wasn't created as part of ::setup()"
        );
        run(Some(&workspace.directory));
        let expected_git_directories = vec![".git", ".git/refs", ".git/objects"];
        assert!(
            expected_git_directories
                .iter()
                .all(|&directory| workspace.directory.join(directory).is_dir()),
            "Newly initialized repo doesn't contain expected directories @ .git"
        );
        workspace.teardown();
    }

    #[test]
    fn init_defaults_to_cwd_when_directory_not_specified() {
        let workspace = TempWorkspace::setup();
        run(None);
        let expected_git_directories = vec![".git", ".git/refs", ".git/objects"];
        assert!(
            expected_git_directories
                .iter()
                .all(|&directory| workspace.directory.join(directory).is_dir()),
            "Newly initialized repo doesn't contain expected directories @ .git"
        );
        workspace.teardown();
    }
}
