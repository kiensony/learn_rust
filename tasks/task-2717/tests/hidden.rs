use task_2717::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_extensions_distinct("src/lib.rs readme docs/notes.md"),
        vec![2u32]
    );
    assert_eq!(
        cli_extensions_distinct("one/../two ./three one//end/"),
        vec![]
    );
    assert_eq!(
        cli_extensions_distinct(".env report.tar.gz tmp"),
        vec![2u32]
    );
    assert_eq!(
        cli_extensions_distinct("a a/z.rs a/b.rs .cargo/config"),
        vec![2u32]
    );
    assert_eq!(
        cli_extensions_distinct("input.release output.log /tmp/ok"),
        vec![3u32, 7u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2717::TRACK_PROOF, 0x52540000 | 2717);
}
