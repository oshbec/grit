use std::{env, fs, path::PathBuf};

use crate::ignore::Ignore;

pub struct Repository {
    directory: PathBuf,
    pub ignore: Ignore,
}

impl Repository {
    pub fn at(directory: Option<PathBuf>) -> Repository {
        let directory = match directory {
            Some(directory) => directory,
            None => env::current_dir().expect("Couldn't identify current working directory"),
        };
        let mut repo = Repository {
            directory,
            ignore: Ignore::new(),
        };
        repo.ignore.add_pattern(String::from(".git"));
        let repo = repo;
        repo
    }

    pub fn init(&self) -> Result<(), std::io::Error> {
        let required_directories = vec![".git", ".git/objects", ".git/refs"];
        for required_directory in required_directories {
            let required_directory = self.directory.join(required_directory);
            fs::create_dir_all(required_directory)?;
        }
        Ok(())
    }

    pub fn list_files(&self) -> Vec<PathBuf> {
        let workspace_files = fs::read_dir(&self.directory)
            .expect("Could not read files in the workspace")
            .map(|dir_entry| dir_entry.unwrap().path())
            .collect();
        self.ignore.ignore_items(workspace_files)
    }
}

#[cfg(test)]
mod tests {
    use super::Repository;
    use std::{env, fs};
    use uuid::Uuid;

    impl Repository {
        pub fn t_generate_temporary() -> Repository {
            let directory_name = format!("temporary_repo_{}", Uuid::new_v4());
            let directory = env::temp_dir().join(directory_name);
            Repository::at(Some(directory))
        }

        pub fn t_destroy(&self) {
            fs::remove_dir_all(&self.directory).expect("Couldn't delete the repository directory");
        }

        pub fn t_exists(&self) -> bool {
            self.directory.is_dir()
        }

        pub fn t_has_expected_git_directories(&self) -> bool {
            let expected_git_directories = vec![".git", ".git/refs", ".git/objects"];
            expected_git_directories
                .iter()
                .all(|&directory| self.directory.join(directory).is_dir())
        }
    }

    #[test]
    fn initializes_repository_in_new_directory() {
        let repository = Repository::t_generate_temporary();
        repository.init().unwrap();
        assert!(
            repository.t_has_expected_git_directories(),
            "Newly initialized repo doesn't contain expected directories @ .git"
        );
        repository.t_destroy();
    }

    #[test]
    fn initializes_repository_in_an_existing_directory() {
        let repository = Repository::t_generate_temporary();
        fs::create_dir(&repository.directory).expect("Couldn't create the temp repository");
        assert!(
            repository.t_exists(),
            "Repo directory wasn't actually created"
        );
        repository.init().unwrap();
        assert!(
            repository.t_has_expected_git_directories(),
            "Newly initialized repo doesn't contain expected directories @ .git"
        );
        repository.t_destroy();
    }

    #[test]
    fn init_defaults_to_cwd_when_directory_not_specified() {
        let temp_directory = Repository::t_generate_temporary().directory;
        fs::create_dir_all(&temp_directory).unwrap();
        let current_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_directory).expect("Couldn't set CWD");
        let repository = Repository::at(None);
        assert_eq!(repository.directory, env::current_dir().unwrap());
        env::set_current_dir(current_dir).unwrap();
        repository.t_destroy();
    }

    #[test]
    fn list_files_that_are_not_ignored() {
        let mut repository = Repository::t_generate_temporary();
        repository.init().unwrap();
        repository.ignore.add_pattern(String::from("LICENSE"));
        let readme_file = repository.directory.join("README");
        let license_file = repository.directory.join("LICENSE");
        fs::write(&readme_file, "This is the README").unwrap();
        fs::write(&license_file, "This is the license").unwrap();
        let commitable_files = repository.list_files();
        assert_eq!(commitable_files.len(), 1);
        assert_eq!(commitable_files.get(0).unwrap().to_owned(), readme_file);
    }
}
