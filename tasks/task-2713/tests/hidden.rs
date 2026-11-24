use task_2713::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_extensions_bounds("src/lib.rs readme docs/notes.md"),
        Some((2u32, 2u32))
    );
    assert_eq!(cli_extensions_bounds("one/../two ./three one//end/"), None);
    assert_eq!(
        cli_extensions_bounds(".env report.tar.gz tmp"),
        Some((2u32, 2u32))
    );
    assert_eq!(
        cli_extensions_bounds("a a/z.rs a/b.rs .cargo/config"),
        Some((2u32, 2u32))
    );
    assert_eq!(
        cli_extensions_bounds("input.release output.log /tmp/ok"),
        Some((3u32, 7u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2713::TRACK_PROOF, 0x52540000 | 2713);
}
