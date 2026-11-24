use task_2705::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_basenames_changes("src/lib.rs readme docs/notes.md"),
        vec![6u32, 8u32]
    );
    assert_eq!(
        cli_basenames_changes("one/../two ./three one//end/"),
        vec![3u32, 5u32, 3u32]
    );
    assert_eq!(
        cli_basenames_changes(".env report.tar.gz tmp"),
        vec![4u32, 13u32, 3u32]
    );
    assert_eq!(
        cli_basenames_changes("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 4u32, 6u32]
    );
    assert_eq!(
        cli_basenames_changes("input.release output.log /tmp/ok"),
        vec![13u32, 10u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2705::TRACK_PROOF, 0x52540000 | 2705);
}
