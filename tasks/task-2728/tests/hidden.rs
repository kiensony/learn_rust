use task_2728::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_steps("src/lib.rs readme docs/notes.md"),
        vec![1u32, 1u32]
    );
    assert_eq!(
        cli_path_depth_steps("one/../two ./three one//end/"),
        vec![1u32, 1u32]
    );
    assert_eq!(
        cli_path_depth_steps(".env report.tar.gz tmp"),
        vec![0u32, 0u32]
    );
    assert_eq!(
        cli_path_depth_steps("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 0u32, 0u32]
    );
    assert_eq!(
        cli_path_depth_steps("input.release output.log /tmp/ok"),
        vec![0u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2728::TRACK_PROOF, 0x52540000 | 2728);
}
