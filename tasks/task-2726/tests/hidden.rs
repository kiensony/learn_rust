use task_2726::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_descending_steps("src/lib.rs readme docs/notes.md"),
        1
    );
    assert_eq!(
        cli_path_depth_descending_steps("one/../two ./three one//end/"),
        1
    );
    assert_eq!(cli_path_depth_descending_steps(".env report.tar.gz tmp"), 0);
    assert_eq!(
        cli_path_depth_descending_steps("a a/z.rs a/b.rs .cargo/config"),
        0
    );
    assert_eq!(
        cli_path_depth_descending_steps("input.release output.log /tmp/ok"),
        0
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2726::TRACK_PROOF, 0x52540000 | 2726);
}
