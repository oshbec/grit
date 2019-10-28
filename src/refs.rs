use std::{env, fs, path::PathBuf};

pub fn update_head(oid: &str) {
    let path = head_path();
    fs::write(path, oid).expect("Couldn't write OID to HEAD");
}

fn head_path() -> PathBuf {
    let current_dir = env::current_dir().expect("Couldn't get current directory");
    current_dir.join(".git").join("HEAD")
}
