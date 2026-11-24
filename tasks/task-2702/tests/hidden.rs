use task_2702::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_basenames_total("src/lib.rs readme docs/notes.md"), 20);
    assert_eq!(cli_basenames_total("one/../two ./three one//end/"), 11);
    assert_eq!(cli_basenames_total(".env report.tar.gz tmp"), 20);
    assert_eq!(cli_basenames_total("a a/z.rs a/b.rs .cargo/config"), 15);
    assert_eq!(cli_basenames_total("input.release output.log /tmp/ok"), 25);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2702::TRACK_PROOF, 0x52540000 | 2702);
}
