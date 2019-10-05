mod common;

use common::TestBed;
use grit::commands::init;

#[test]
fn initializes_repository_in_existing_directory() {
    let test_bed = TestBed::setup();
    assert_eq!(
        test_bed.workspace().is_dir(),
        true,
        "The workspace directory wasn't created as part of ::setup()"
    );
    init(Some(&test_bed.workspace()));
    let expected_git_directories = vec![".git", ".git/refs", ".git/objects"];
    assert!(
        expected_git_directories
            .iter()
            .all(|&directory| test_bed.workspace().join(directory).is_dir()),
        "Newly initialized repo doesn't contain expected directories @ .git"
    );
    test_bed.teardown();
}

#[test]
fn init_defaults_to_cwd_when_directory_not_specified() {
    let test_bed = TestBed::setup();
    init(None);
    let expected_git_directories = vec![".git", ".git/refs", ".git/objects"];
    assert!(
        expected_git_directories
            .iter()
            .all(|&directory| test_bed.workspace().join(directory).is_dir()),
        "Newly initialized repo doesn't contain expected directories @ .git"
    );
    test_bed.teardown();
}
