use task_2727::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_distinct("src/lib.rs readme docs/notes.md"),
        vec![1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_distinct("one/../two ./three one//end/"),
        vec![1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_distinct(".env report.tar.gz tmp"),
        vec![1u32]
    );
    assert_eq!(
        cli_path_depth_distinct("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_distinct("input.release output.log /tmp/ok"),
        vec![1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2727::TRACK_PROOF, 0x52540000 | 2727);
}
