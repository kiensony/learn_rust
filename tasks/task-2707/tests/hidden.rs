use task_2707::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_basenames_distinct("src/lib.rs readme docs/notes.md"),
        vec![6u32, 8u32]
    );
    assert_eq!(
        cli_basenames_distinct("one/../two ./three one//end/"),
        vec![3u32, 5u32]
    );
    assert_eq!(
        cli_basenames_distinct(".env report.tar.gz tmp"),
        vec![3u32, 4u32, 13u32]
    );
    assert_eq!(
        cli_basenames_distinct("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 4u32, 6u32]
    );
    assert_eq!(
        cli_basenames_distinct("input.release output.log /tmp/ok"),
        vec![2u32, 10u32, 13u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2707::TRACK_PROOF, 0x52540000 | 2707);
}
