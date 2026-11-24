use task_2704::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_basenames_peaks("src/lib.rs readme docs/notes.md"),
        vec![6u32, 8u32]
    );
    assert_eq!(
        cli_basenames_peaks("one/../two ./three one//end/"),
        vec![3u32, 5u32]
    );
    assert_eq!(
        cli_basenames_peaks(".env report.tar.gz tmp"),
        vec![4u32, 13u32]
    );
    assert_eq!(
        cli_basenames_peaks("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 4u32, 6u32]
    );
    assert_eq!(
        cli_basenames_peaks("input.release output.log /tmp/ok"),
        vec![13u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2704::TRACK_PROOF, 0x52540000 | 2704);
}
