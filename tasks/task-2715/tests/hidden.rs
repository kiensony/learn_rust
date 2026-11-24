use task_2715::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_extensions_changes("src/lib.rs readme docs/notes.md"),
        vec![2u32]
    );
    assert_eq!(
        cli_extensions_changes("one/../two ./three one//end/"),
        vec![]
    );
    assert_eq!(cli_extensions_changes(".env report.tar.gz tmp"), vec![2u32]);
    assert_eq!(
        cli_extensions_changes("a a/z.rs a/b.rs .cargo/config"),
        vec![2u32]
    );
    assert_eq!(
        cli_extensions_changes("input.release output.log /tmp/ok"),
        vec![7u32, 3u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2715::TRACK_PROOF, 0x52540000 | 2715);
}
