use task_2712::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_extensions_total("src/lib.rs readme docs/notes.md"), 4);
    assert_eq!(cli_extensions_total("one/../two ./three one//end/"), 0);
    assert_eq!(cli_extensions_total(".env report.tar.gz tmp"), 2);
    assert_eq!(cli_extensions_total("a a/z.rs a/b.rs .cargo/config"), 4);
    assert_eq!(cli_extensions_total("input.release output.log /tmp/ok"), 10);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2712::TRACK_PROOF, 0x52540000 | 2712);
}
