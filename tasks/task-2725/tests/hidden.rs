use task_2725::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_changes("src/lib.rs readme docs/notes.md"),
        vec![2u32, 1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_changes("one/../two ./three one//end/"),
        vec![2u32, 1u32, 2u32]
    );
    assert_eq!(cli_path_depth_changes(".env report.tar.gz tmp"), vec![1u32]);
    assert_eq!(
        cli_path_depth_changes("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_changes("input.release output.log /tmp/ok"),
        vec![1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2725::TRACK_PROOF, 0x52540000 | 2725);
}
