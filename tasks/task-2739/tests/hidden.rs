use task_2739::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_rust_sources_longest_run("src/lib.rs readme docs/notes.md"),
        1
    );
    assert_eq!(
        cli_rust_sources_longest_run("one/../two ./three one//end/"),
        0
    );
    assert_eq!(cli_rust_sources_longest_run(".env report.tar.gz tmp"), 0);
    assert_eq!(
        cli_rust_sources_longest_run("a a/z.rs a/b.rs .cargo/config"),
        1
    );
    assert_eq!(
        cli_rust_sources_longest_run("input.release output.log /tmp/ok"),
        0
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2739::TRACK_PROOF, 0x52540000 | 2739);
}
