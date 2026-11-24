use task_2740::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cli_rust_sources_reverse("src/lib.rs readme docs/notes.md"),
        vec![0u32]
    );
    assert_eq!(
        cli_rust_sources_reverse("one/../two ./three one//end/"),
        vec![]
    );
    assert_eq!(cli_rust_sources_reverse(".env report.tar.gz tmp"), vec![]);
    assert_eq!(
        cli_rust_sources_reverse("a a/z.rs a/b.rs .cargo/config"),
        vec![2u32, 1u32]
    );
    assert_eq!(
        cli_rust_sources_reverse("input.release output.log /tmp/ok"),
        vec![]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2740::TRACK_PROOF, 0x52540000 | 2740);
}
