use std::{env, fs, path::PathBuf};

/// Create an empty Git repository
pub fn run(directory: Option<&PathBuf>) {
    let current_dir = env::current_dir().expect("Couldn't identify current working directory");
    let directory = directory.unwrap_or(&current_dir);
    let required_directories = vec![".git", ".git/objects", ".git/refs"];
    for required_directory in required_directories {
        let required_directory = directory.join(required_directory);
        fs::create_dir_all(required_directory).expect("Could not create required directory");
    }
    println!("Initialized the git repo at {:?}", &directory);
}
