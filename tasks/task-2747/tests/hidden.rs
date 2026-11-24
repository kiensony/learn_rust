use task_2747::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_extensions_run_ledger("a.txt"), vec![(3u32, 1usize)]);
    assert_eq!(
        cli_extensions_run_ledger("src/lib.rs readme docs/notes.md"),
        vec![(2u32, 2usize)]
    );
    assert_eq!(
        cli_extensions_run_ledger("one/../two ./three one//end/"),
        vec![]
    );
    assert_eq!(
        cli_extensions_run_ledger(".env report.tar.gz tmp"),
        vec![(2u32, 1usize)]
    );
    assert_eq!(
        cli_extensions_run_ledger("a a/z.rs a/b.rs .cargo/config"),
        vec![(2u32, 2usize)]
    );
    assert_eq!(
        cli_extensions_run_ledger("input.release output.log /tmp/ok"),
        vec![(7u32, 1usize), (3u32, 1usize)]
    );
    assert_eq!(
        cli_extensions_run_ledger("input.release output.log /tmp/ok"),
        vec![(7u32, 1usize), (3u32, 1usize)]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2747::TRACK_PROOF, 0x52540000 | 2747);
}
