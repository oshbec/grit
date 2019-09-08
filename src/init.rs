use std::fs;
use std::path::PathBuf;

pub fn run<P: Into<PathBuf>>(directory: P) {
    let mut directory = directory.into();
    directory.push(".git");
    fs::create_dir_all(&directory).unwrap();
    directory.push("objects");
    fs::create_dir_all(&directory).unwrap();
    directory.pop();
    directory.push("refs");
    fs::create_dir_all(&directory).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, fs};

    #[test]
    fn initializes_repository_in_new_directory() {
        // ARRANGE
        let mut directory = env::temp_dir();
        directory.push("repo");
        let directory = directory.as_path();
        // ACT
        assert_eq!(directory.is_dir(), false);
        run(&directory);
        // ASSERT
        for check_directory in &[".git", ".git/refs", ".git/objects"] {
            let mut expected_directory = PathBuf::from(directory);
            expected_directory.push(check_directory);
            assert!(expected_directory.is_dir());
        }
        // CLEANUP
        fs::remove_dir_all(directory).unwrap();
        assert_eq!(directory.is_dir(), false);
    }

    #[test]
    fn initializes_repository_in_an_existing_directory() {
        // ARRANGE
        let mut directory = env::temp_dir();
        directory.push("some_dir");
        let directory = directory.as_path();
        fs::create_dir(directory).unwrap();
        // ACT
        assert_eq!(directory.is_dir(), true);
        run(&directory);
        // ASSERT
        for check_directory in &[".git", ".git/refs", ".git/objects"] {
            let mut expected_directory = PathBuf::from(directory);
            expected_directory.push(check_directory);
            assert!(expected_directory.is_dir());
        }
        // CLEANUP
        fs::remove_dir_all(directory).unwrap();
        assert_eq!(directory.is_dir(), false);
    }
}
