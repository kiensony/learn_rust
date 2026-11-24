use task_2711::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_extensions_trace("src/lib.rs readme docs/notes.md"),
        vec![2u32, 2u32]
    );
    assert_eq!(cli_extensions_trace("one/../two ./three one//end/"), vec![]);
    assert_eq!(cli_extensions_trace(".env report.tar.gz tmp"), vec![2u32]);
    assert_eq!(
        cli_extensions_trace("a a/z.rs a/b.rs .cargo/config"),
        vec![2u32, 2u32]
    );
    assert_eq!(
        cli_extensions_trace("input.release output.log /tmp/ok"),
        vec![7u32, 3u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2711::TRACK_PROOF, 0x52540000 | 2711);
}
