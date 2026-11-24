use task_2714::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_extensions_peaks("src/lib.rs readme docs/notes.md"),
        vec![2u32]
    );
    assert_eq!(cli_extensions_peaks("one/../two ./three one//end/"), vec![]);
    assert_eq!(cli_extensions_peaks(".env report.tar.gz tmp"), vec![2u32]);
    assert_eq!(
        cli_extensions_peaks("a a/z.rs a/b.rs .cargo/config"),
        vec![2u32]
    );
    assert_eq!(
        cli_extensions_peaks("input.release output.log /tmp/ok"),
        vec![7u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2714::TRACK_PROOF, 0x52540000 | 2714);
}
