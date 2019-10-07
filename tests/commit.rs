mod common;
use grit::commands::{commit, init};

use common::TestBed;

use time;

#[test]
fn creates_objects_found_in_real_git_commit() {
    let test_bed = TestBed::setup();

    init(Some(&test_bed.workspace()));

    test_bed.create_file("README", "This is the README");
    // test_bed.create_file("LICENSE", "This is the license");

    let right_now = time::now();

    let message = "It is a commit!";

    commit(message, Some(right_now));

    let right_now = format!(
        "{}",
        right_now
            .strftime("%Y-%m-%d")
            .expect("Could not format date")
    );

    test_bed.git_command(vec!["init"]);
    test_bed.git_command(vec!["add", "."]);
    test_bed.git_command(vec![
        "commit",
        "-m",
        "It is a commit!",
        "--date",
        &right_now,
    ]);

    assert!(
        test_bed.contained_by_twin(".git/objects"),
        "Files in workspace not contained by twin, take a look:\n{:?}",
        test_bed.root
    );

    test_bed.teardown();
}
