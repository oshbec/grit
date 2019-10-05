mod common;
use grit::commands::{commit, init};

use common::TestBed;

#[test]
fn creates_objects_found_in_real_git_commit() {
    let test_bed = TestBed::setup();

    init(Some(&test_bed.workspace()));

    test_bed.create_file("README", "This is the README");
    // test_bed.create_file("LICENSE", "This is the license");

    commit();

    test_bed.git_command(vec!["init"]);
    test_bed.git_command(vec!["add", "."]);
    test_bed.git_command(vec!["commit", "-m \"First commit\""]);

    assert!(
        test_bed.contained_by_twin(".git/objects"),
        "Files in workspace not contained by twin, take a look:\n{:?}",
        test_bed.root
    );

    test_bed.teardown();
}
