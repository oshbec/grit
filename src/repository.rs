use std::path::PathBuf;
use std::{env, fs};

pub struct Repository {
    directory: PathBuf,
}

impl Repository {
    pub fn init(&self) -> Result<(), std::io::Error> {
        let required_directories = vec![".git", ".git/objects", ".git/refs"];
        for required_directory in required_directories {
            let required_directory = self.directory.join(required_directory);
            fs::create_dir_all(required_directory)?;
        }
        Ok(())
    }

    pub fn at(directory: Option<PathBuf>) -> Repository {
        let directory = match directory {
            Some(directory) => directory,
            None => env::current_dir().expect("Couldn't identify current working directory"),
        };
        Repository { directory }
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
            assert!(
                !directory.is_dir(),
                "Temporary repo directory already exists"
            );
            Repository { directory }
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
}
