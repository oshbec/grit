use std::fs;
use std::path::Path;

pub fn run<P: AsRef<Path>>(directory: P) {
    let directory = directory.as_ref();
    let required_directories = &[".git", ".git/objects", ".git/refs"];
    for required_directory in required_directories {
        let required_directory = directory.join(required_directory);
        fs::create_dir_all(required_directory)
            .expect("Could'nt create a required repository directory")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, fs};

    #[test]
    fn initializes_repository_in_new_directory() {
        // ARRANGE
        let directory = env::temp_dir().join("repo");
        // ACT
        assert_eq!(directory.is_dir(), false);
        run(&directory);
        // ASSERT
        for check_directory in &[".git", ".git/refs", ".git/objects"] {
            let expected_directory = directory.join(check_directory);
            assert!(expected_directory.is_dir());
        }
        // CLEANUP
        fs::remove_dir_all(&directory).expect("Couldn't clean up the temp repository");
        assert_eq!(directory.is_dir(), false);
    }

    #[test]
    fn initializes_repository_in_an_existing_directory() {
        // ARRANGE
        let directory = env::temp_dir().join("some_dir");
        fs::create_dir(&directory).expect("Couldn't create the temp repository");
        // ACT
        assert_eq!(directory.is_dir(), true);
        run(&directory);
        // ASSERT
        for check_directory in &[".git", ".git/refs", ".git/objects"] {
            let expected_directory = directory.join(check_directory);
            assert!(expected_directory.is_dir());
        }
        // CLEANUP
        fs::remove_dir_all(&directory).expect("Couldn't clean up the temp repository");
        assert_eq!(directory.is_dir(), false);
    }
}
