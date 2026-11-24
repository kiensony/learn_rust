use task_2733::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_rust_sources_bounds("src/lib.rs readme docs/notes.md"),
        Some((0u32, 0u32))
    );
    assert_eq!(
        cli_rust_sources_bounds("one/../two ./three one//end/"),
        None
    );
    assert_eq!(cli_rust_sources_bounds(".env report.tar.gz tmp"), None);
    assert_eq!(
        cli_rust_sources_bounds("a a/z.rs a/b.rs .cargo/config"),
        Some((1u32, 2u32))
    );
    assert_eq!(
        cli_rust_sources_bounds("input.release output.log /tmp/ok"),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2733::TRACK_PROOF, 0x52540000 | 2733);
}
