use task_2745::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_stems_changes("src/lib.rs readme docs/notes.md"),
        vec![3u32, 6u32, 5u32]
    );
    assert_eq!(
        cli_stems_changes("one/../two ./three one//end/"),
        vec![3u32, 5u32, 3u32]
    );
    assert_eq!(
        cli_stems_changes(".env report.tar.gz tmp"),
        vec![4u32, 10u32, 3u32]
    );
    assert_eq!(
        cli_stems_changes("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 6u32]
    );
    assert_eq!(
        cli_stems_changes("input.release output.log /tmp/ok"),
        vec![5u32, 6u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2745::TRACK_PROOF, 0x52540000 | 2745);
}
