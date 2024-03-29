use crate::{
    ignore::Ignore,
    objects::{self, Blob, Commit, Object, Tree},
    refs,
};
use std::{env, fs, path::PathBuf};

/// Record changes to the repository
pub fn run(message: &str) -> Result<String, String> {
    let current_dir = env::current_dir().expect("Couldn't determine current directory");
    let ignore: Ignore = Default::default();
    let files_to_commit = list_files(&current_dir, &ignore);

    let blobs: Vec<Blob> = files_to_commit
        .iter()
        .map(|file| {
            let blob = Blob::from_file(file);
            objects::write(&blob).expect("Couldn't write blob to git database");
            blob
        })
        .collect();

    let tree = Tree::from_blobs(blobs);
    objects::write(&tree).expect("Couldn't write tree to git database");

    let commit = Commit::new(tree.id(), message);
    objects::write(&commit).expect("Couldn't write the commit to git database");

    refs::update_head(&commit.id());

    Ok(commit.id())
}

fn list_files(workspace: &PathBuf, ignore: &Ignore) -> Vec<PathBuf> {
    let workspace_files = fs::read_dir(workspace)
        .expect("Could not read files in the workspace")
        .map(|dir_entry| dir_entry.expect("Can't get dir_entry").path())
        .collect();
    ignore.ignore_items(workspace_files)
}
