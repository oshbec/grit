mod common;
use grit::commands::{commit, init};

use common::TestBed;

use std::env;

use chrono::Local;

#[test]
fn creates_objects_found_in_real_git_commit() {
    let test_bed = TestBed::setup();

    init(Some(&test_bed.workspace()));

    test_bed.create_file("README", "This is the README");
    // test_bed.create_file("LICENSE", "This is the license");

    let message = "It is a commit!";
    let right_now = Local::now().format("%s %z").to_string();

    env::set_var("GIT_AUTHOR_DATE", &right_now);
    env::set_var("GIT_COMMITTER_DATE", &right_now);

    commit(message);

    test_bed.git_command(vec!["init"]);
    test_bed.git_command(vec!["add", "."]);
    test_bed.git_command(vec!["commit", "-m", "It is a commit!"]);

    assert!(
        test_bed.contained_by_twin(".git/objects"),
        "Files in workspace not contained by twin, take a look:\n{:?}",
        test_bed.root
    );

    test_bed.teardown();
}
