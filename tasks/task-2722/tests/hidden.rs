use task_2722::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_path_depth_total("src/lib.rs readme docs/notes.md"), 5);
    assert_eq!(cli_path_depth_total("one/../two ./three one//end/"), 5);
    assert_eq!(cli_path_depth_total(".env report.tar.gz tmp"), 3);
    assert_eq!(cli_path_depth_total("a a/z.rs a/b.rs .cargo/config"), 7);
    assert_eq!(cli_path_depth_total("input.release output.log /tmp/ok"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2722::TRACK_PROOF, 0x52540000 | 2722);
}
