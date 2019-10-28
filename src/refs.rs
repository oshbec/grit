use std::{env, fs, path::PathBuf};

pub fn update_head(oid: &str) {
    fs::write(head_path(), oid).expect("Couldn't write OID to HEAD");
}

pub fn read_head() -> Option<String> {
    match fs::read_to_string(head_path()) {
        Ok(head) => Some(head),
        Err(_) => None,
    }
}

fn head_path() -> PathBuf {
    let current_dir = env::current_dir().expect("Couldn't get current directory");
    current_dir.join(".git").join("HEAD")
}
