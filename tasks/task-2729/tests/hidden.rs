use task_2729::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_longest_run("src/lib.rs readme docs/notes.md"),
        1
    );
    assert_eq!(
        cli_path_depth_longest_run("one/../two ./three one//end/"),
        1
    );
    assert_eq!(cli_path_depth_longest_run(".env report.tar.gz tmp"), 3);
    assert_eq!(
        cli_path_depth_longest_run("a a/z.rs a/b.rs .cargo/config"),
        3
    );
    assert_eq!(
        cli_path_depth_longest_run("input.release output.log /tmp/ok"),
        2
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2729::TRACK_PROOF, 0x52540000 | 2729);
}
