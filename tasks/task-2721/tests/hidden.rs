use task_2721::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_trace("src/lib.rs readme docs/notes.md"),
        vec![2u32, 1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_trace("one/../two ./three one//end/"),
        vec![2u32, 1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_trace(".env report.tar.gz tmp"),
        vec![1u32, 1u32, 1u32]
    );
    assert_eq!(
        cli_path_depth_trace("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 2u32, 2u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_trace("input.release output.log /tmp/ok"),
        vec![1u32, 1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2721::TRACK_PROOF, 0x52540000 | 2721);
}
