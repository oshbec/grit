use std::{env, fs, path::PathBuf, process::Command};
use uuid::Uuid;

use grit::compression;

#[derive(Debug)]
pub struct TestBed {
    pub root: PathBuf,
}

#[allow(dead_code)]
impl TestBed {
    pub fn setup() -> TestBed {
        let root = env::temp_dir().join(format!("grit_test/{}", Uuid::new_v4()));
        let test_bed = TestBed { root };
        fs::create_dir_all(test_bed.workspace()).expect("Couldn't create workspace directory");
        fs::create_dir_all(test_bed.twin()).expect("Couldn't create twin directory");
        env::set_current_dir(test_bed.workspace()).expect("Couldn't set workspace to CWD");
        env::set_var("GIT_AUTHOR_NAME", "Count Dracula");
        env::set_var("GIT_AUTHOR_EMAIL", "count@dracula");
        env::set_var("GIT_COMMITTER_NAME", "Count Dracula");
        env::set_var("GIT_COMMITTER_EMAIL", "count@dracula");
        test_bed
    }

    pub fn teardown(&self) {
        fs::remove_dir_all(&self.root).expect("Couldn't delete the test workspace");
        assert!(!&self.root.exists());
    }

    // Path to the workspace directory, where `grit` operates
    pub fn workspace(&self) -> PathBuf {
        self.root.join("workspace")
    }

    // Path to the twin directory, where `git` operates
    pub fn twin(&self) -> PathBuf {
        self.root.join("twin")
    }

    // Sugar to get both workspace and twin for iteration in less boilerplate
    fn test_parallels(&self) -> Vec<PathBuf> {
        vec![self.workspace(), self.twin()]
    }

    // Create a file to both `workspace` and twin
    pub fn create_file(&self, relative_path: &str, contents: &str) {
        for test_parallel in self.test_parallels() {
            let path = test_parallel.join(relative_path);
            simple_write_file(&path, contents);
        }
    }

    // Create a directory in both `workspace` and `twin`
    pub fn create_directory(&self, relative_path: &str) {
        for test_parallel in self.test_parallels() {
            let path = test_parallel.join(relative_path);
            fs::create_dir_all(&path).expect("Couldn't create duplicate directory");
        }
    }

    // Run a `git` command in `twin`
    pub fn git_command(&self, args: Vec<&str>) {
        Command::new("git")
            .args(args)
            .current_dir(&self.twin())
            .output()
            .expect("Git command failed");
    }

    // Finds the twin version of a path in the TestBed
    pub fn find_twin(&self, path: &PathBuf) -> PathBuf {
        let workspace = self.workspace().to_str().unwrap().to_owned();
        let twin = self.twin().to_str().unwrap().to_owned();

        // Stringify the path, and replace the part matching the workspace with the twin
        let converted_path = path.to_str().unwrap().to_owned().replace(&workspace, &twin);

        PathBuf::from(converted_path)
    }

    pub fn contained_by_twin(&self, path: &str) -> bool {
        let path_in_workspace = self.workspace().join(path);
        let path_in_twin = self.find_twin(&path_in_workspace);

        if path_in_workspace.is_dir() {
            let workspace_files = descendent_files(&path_in_workspace);
            return workspace_files.iter().all(|file| {
                let twin = self.find_twin(file);
                let identical = files_are_identical(file, &twin);
                if !identical {
                    inspect_parallels(&path_in_workspace, &path_in_twin);
                }
                identical
            });
        }

        if path_in_workspace.is_file() {
            return files_are_identical(&path_in_workspace, &path_in_twin);
        }

        false
    }
}

// Recursively delve into directories and collect every file found
fn descendent_files(directory: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let dir_entries = match directory.read_dir() {
        Ok(dir_entries) => dir_entries,
        Err(_) => return vec![], // No files found do to read_dir error
    };

    for dir_entry in dir_entries {
        let dir_entry = match dir_entry {
            Ok(dir_entry) => dir_entry.path(),
            Err(_) => continue, // Skip this dir_entry
        };
        if dir_entry.is_dir() {
            let mut found_files = descendent_files(&dir_entry);
            files.append(&mut found_files);
        }
        if dir_entry.is_file() {
            files.push(dir_entry)
        }
    }

    files
}

pub fn files_are_identical(first: &PathBuf, second: &PathBuf) -> bool {
    let first_file = match simple_read_file(first) {
        Ok(file) => file,
        Err(_) => {
            println!("Couldn't read file: {:?}", first);
            return false;
        }
    };
    let second_file = match simple_read_file(second) {
        Ok(file) => file,
        Err(_) => {
            println!("Couldn't read file: {:?}", second);
            return false;
        }
    };

    let they_match = first_file == second_file;
    if !they_match {
        println!(
            "Files did not match :-(\n\n{:?}\n\n{:?}\n\n{:?}\n\n{:?}",
            first, first_file, second, second_file
        );
        inspect_parallels(first, second);
    }
    they_match
}

pub fn inspect_parallels(first: &PathBuf, second: &PathBuf) {
    if first.is_file() && second.is_file() {
        println!("=== COMPARING FILES ===");
        simple_print_file(first);
        println!("===       VS.       ===");
        simple_print_file(second);
        println!("===       END       ===");
    }
    if first.is_dir() && second.is_dir() {
        let first_files = descendent_files(first);
        let second_files = descendent_files(second);
        println!("=== FILES IN FIRST DIRECTORY ===");
        for file in first_files {
            simple_print_file(&file);
        }
        println!("=== FILES IN SECOND DIRECTORY ===");
        for file in second_files {
            simple_print_file(&file);
        }
    }
}

fn simple_print_file(path: &PathBuf) {
    let contents = match simple_read_file(path) {
        Ok(contents) => contents,
        _ => panic!("Couldnt read file {:?}", path),
    };
    println!(
        "File at `{:?}` contains:\n{}",
        path,
        String::from_utf8_lossy(&contents)
    );
}

// Reads a file, and returns a decrompressed version if that operation is successful
// Could be very bad if a file starts with the right (wrong) bytes, might need to rethink if problems arise
fn simple_read_file(path: &PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let file = fs::read(path)?;
    match compression::decompress(&file) {
        Ok(decompressed) => Ok(decompressed),
        Err(_) => Ok(file),
    }
}

// Creates a file based on path and contents
// Will create parent directory for the file if it doesn't yet exist
fn simple_write_file(path: &PathBuf, contents: &str) {
    let parent_directory = path
        .parent()
        .expect("Couldn't determine parent directory from file path");
    fs::create_dir_all(parent_directory).expect("Could not create parent directory for file");
    fs::write(path, &contents).expect("Couldn't write file");
}

#[cfg(feature = "helper_tests")]
mod tests {
    use super::*;

    #[test]
    fn writes_file_to_twin_and_workspace() {
        let test_bed = TestBed::setup();
        test_bed.create_file("README", "This is a readme");
        test_bed.contained_by_twin("README");
        test_bed.teardown();
    }

    #[test]
    fn file_doesnt_match_twin_when_twin_doesnt_exist() {
        let test_bed = TestBed::setup();
        fs::write(test_bed.workspace().join("README"), "Hello, friend").unwrap();
        assert_eq!(test_bed.contained_by_twin("README"), false);
        test_bed.teardown();
    }

    #[test]
    fn file_doesnt_match_existing_twin() {
        let test_bed = TestBed::setup();
        fs::write(test_bed.workspace().join("README"), "Hello, friend").unwrap();
        fs::write(test_bed.twin().join("README"), "Goodbye, friend").unwrap();
        assert_eq!(test_bed.contained_by_twin("README"), false);
        test_bed.teardown();
    }

    #[test]
    fn workspace_directory_contents_not_found_in_twin() {
        let test_bed = TestBed::setup();
        test_bed.create_directory("something");
        fs::write(
            test_bed.workspace().join("something/README"),
            "Hello, friend",
        )
        .unwrap();
        assert_eq!(test_bed.contained_by_twin("something"), false);
        test_bed.teardown();
    }

    #[test]
    fn workspace_directory_file_contents_dont_match_in_twin() {
        let test_bed = TestBed::setup();
        test_bed.create_directory("something");
        fs::write(
            test_bed.workspace().join("something/README"),
            "Hello, friend",
        )
        .unwrap();
        fs::write(test_bed.twin().join("something/README"), "Goodbye, friend").unwrap();
        assert_eq!(test_bed.contained_by_twin("something"), false);
        test_bed.teardown();
    }

    #[test]
    fn git_commands_run_in_twin() {
        let test_bed = TestBed::setup();
        test_bed.git_command(vec!["init"]);
        let git_path = test_bed.twin().join(".git");
        assert!(git_path.exists());
    }
}
