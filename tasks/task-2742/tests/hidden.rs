use task_2742::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_stems_total("src/lib.rs readme docs/notes.md"), 14);
    assert_eq!(cli_stems_total("one/../two ./three one//end/"), 11);
    assert_eq!(cli_stems_total(".env report.tar.gz tmp"), 17);
    assert_eq!(cli_stems_total("a a/z.rs a/b.rs .cargo/config"), 9);
    assert_eq!(cli_stems_total("input.release output.log /tmp/ok"), 13);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2742::TRACK_PROOF, 0x52540000 | 2742);
}
