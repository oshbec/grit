use std::{env, fs, path::PathBuf};

use crate::{
    ignore::Ignore,
    objects::{self, Blob, Tree},
};

pub fn run() {
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
}

fn list_files(workspace: &PathBuf, ignore: &Ignore) -> Vec<PathBuf> {
    let workspace_files = fs::read_dir(workspace)
        .expect("Could not read files in the workspace")
        .map(|dir_entry| dir_entry.unwrap().path())
        .collect();
    ignore.ignore_items(workspace_files)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_utilities::TestWorkspace;

    #[test]
    fn list_files_that_are_not_ignored() {
        let workspace = TestWorkspace::setup();

        let mut ignore = Ignore::new();
        ignore.add_pattern(String::from("LICENSE"));

        let readme_file = workspace.directory.join("README");
        fs::write(&readme_file, "This is the README").unwrap();
        let license_file = workspace.directory.join("LICENSE");
        fs::write(&license_file, "This is the license").unwrap();

        let commitable_files = super::list_files(&workspace.directory, &ignore);
        assert_eq!(commitable_files.len(), 1);
        assert_eq!(commitable_files.get(0).unwrap().to_owned(), readme_file);

        workspace.teardown();
    }

}
