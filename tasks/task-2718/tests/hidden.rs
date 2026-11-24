use task_2718::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_extensions_steps("src/lib.rs readme docs/notes.md"),
        vec![0u32]
    );
    assert_eq!(cli_extensions_steps("one/../two ./three one//end/"), vec![]);
    assert_eq!(cli_extensions_steps(".env report.tar.gz tmp"), vec![]);
    assert_eq!(
        cli_extensions_steps("a a/z.rs a/b.rs .cargo/config"),
        vec![0u32]
    );
    assert_eq!(
        cli_extensions_steps("input.release output.log /tmp/ok"),
        vec![4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2718::TRACK_PROOF, 0x52540000 | 2718);
}
