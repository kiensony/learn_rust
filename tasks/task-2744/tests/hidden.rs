use task_2744::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_stems_peaks("src/lib.rs readme docs/notes.md"),
        vec![3u32, 6u32]
    );
    assert_eq!(
        cli_stems_peaks("one/../two ./three one//end/"),
        vec![3u32, 5u32]
    );
    assert_eq!(cli_stems_peaks(".env report.tar.gz tmp"), vec![4u32, 10u32]);
    assert_eq!(
        cli_stems_peaks("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 6u32]
    );
    assert_eq!(
        cli_stems_peaks("input.release output.log /tmp/ok"),
        vec![5u32, 6u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2744::TRACK_PROOF, 0x52540000 | 2744);
}
