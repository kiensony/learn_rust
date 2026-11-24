use task_2732::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_rust_sources_total("src/lib.rs readme docs/notes.md"), 0);
    assert_eq!(cli_rust_sources_total("one/../two ./three one//end/"), 0);
    assert_eq!(cli_rust_sources_total(".env report.tar.gz tmp"), 0);
    assert_eq!(cli_rust_sources_total("a a/z.rs a/b.rs .cargo/config"), 3);
    assert_eq!(
        cli_rust_sources_total("input.release output.log /tmp/ok"),
        0
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2732::TRACK_PROOF, 0x52540000 | 2732);
}
