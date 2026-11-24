use task_2738::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_rust_sources_steps("src/lib.rs readme docs/notes.md"),
        vec![]
    );
    assert_eq!(
        cli_rust_sources_steps("one/../two ./three one//end/"),
        vec![]
    );
    assert_eq!(cli_rust_sources_steps(".env report.tar.gz tmp"), vec![]);
    assert_eq!(
        cli_rust_sources_steps("a a/z.rs a/b.rs .cargo/config"),
        vec![1u32]
    );
    assert_eq!(
        cli_rust_sources_steps("input.release output.log /tmp/ok"),
        vec![]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2738::TRACK_PROOF, 0x52540000 | 2738);
}
