use task_2716::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_extensions_descending_steps("src/lib.rs readme docs/notes.md"),
        0
    );
    assert_eq!(
        cli_extensions_descending_steps("one/../two ./three one//end/"),
        0
    );
    assert_eq!(cli_extensions_descending_steps(".env report.tar.gz tmp"), 0);
    assert_eq!(
        cli_extensions_descending_steps("a a/z.rs a/b.rs .cargo/config"),
        0
    );
    assert_eq!(
        cli_extensions_descending_steps("input.release output.log /tmp/ok"),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2716::TRACK_PROOF, 0x52540000 | 2716);
}
