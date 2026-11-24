use task_2730::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_path_depth_reverse("src/lib.rs readme docs/notes.md"),
        vec![2u32, 1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_reverse("one/../two ./three one//end/"),
        vec![2u32, 1u32, 2u32]
    );
    assert_eq!(
        cli_path_depth_reverse(".env report.tar.gz tmp"),
        vec![1u32, 1u32, 1u32]
    );
    assert_eq!(
        cli_path_depth_reverse("a a/z.rs a/b.rs .cargo/config"),
        vec![2u32, 2u32, 2u32, 1u32]
    );
    assert_eq!(
        cli_path_depth_reverse("input.release output.log /tmp/ok"),
        vec![2u32, 1u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2730::TRACK_PROOF, 0x52540000 | 2730);
}
