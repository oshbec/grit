use std::path::PathBuf;
use std::{env, fs};
use uuid::Uuid;

pub struct Repository {
    directory: PathBuf,
}

impl Repository {
    pub fn init(&self) {
        let required_directories = vec![".git", ".git/objects", ".git/refs"];
        for required_directory in required_directories {
            let required_directory = self.directory.join(required_directory);
            fs::create_dir_all(required_directory)
                .expect("Couldn't create a required repository directory")
        }
    }

    pub fn at<P: Into<PathBuf>>(directory: P) -> Repository {
        Repository {
            directory: directory.into(),
        }
    }
}

#[cfg(test)]
impl Repository {
    pub fn t_generate_temporary() -> Repository {
        let directory_name = format!("temporary_repo_{}", Uuid::new_v4());
        let directory = env::temp_dir().join(directory_name);
        assert_eq!(
            directory.is_dir(),
            false,
            "Temporary repo directory already exists"
        );
        Repository { directory }
    }

    pub fn t_destroy(&self) {
        fs::remove_dir_all(&self.directory).expect("Couldn't delete the repository directory");
        assert_eq!(
            self.directory.is_dir(),
            false,
            "Temporary repo directory wasn't actually deleted"
        );
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

#[cfg(test)]
mod tests {
    use super::Repository;
    use std::fs;

    #[test]
    fn initializes_repository_in_new_directory() {
        let repository = Repository::t_generate_temporary();
        repository.init();
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
        repository.init();
        assert!(
            repository.t_has_expected_git_directories(),
            "Newly initialized repo doesn't contain expected directories @ .git"
        );
        repository.t_destroy();
    }
}
