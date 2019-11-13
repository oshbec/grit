use std::{env, fs, path::PathBuf};

/// Create an empty Git repository
pub fn run(directory: Option<&PathBuf>) {
    let current_dir = env::current_dir().expect("Couldn't identify current working directory");
    let directory = directory.unwrap_or(&current_dir);

    let required_git_directories = vec![".git", ".git/objects", ".git/refs"];
    for required_git_directory in required_git_directories {
        let required_git_directory = directory.join(required_git_directory);
        fs::create_dir_all(required_git_directory).expect("Could not create required directory");
    }
    println!("Initialized the git repo at {:?}", &directory);
}
