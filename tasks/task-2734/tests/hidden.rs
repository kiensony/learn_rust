use task_2734::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_rust_sources_peaks("src/lib.rs readme docs/notes.md"),
        vec![0u32]
    );
    assert_eq!(
        cli_rust_sources_peaks("one/../two ./three one//end/"),
        vec![]
    );
    assert_eq!(cli_rust_sources_peaks(".env report.tar.gz tmp"), vec![]);
    assert_eq!(
        cli_rust_sources_peaks("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32, 2u32]
    );
    assert_eq!(
        cli_rust_sources_peaks("input.release output.log /tmp/ok"),
        vec![]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2734::TRACK_PROOF, 0x52540000 | 2734);
}
