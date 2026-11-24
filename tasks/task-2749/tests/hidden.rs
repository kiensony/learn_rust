use task_2749::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_rust_sources_frequent_readings("a.txt", 0), vec![]);
    assert_eq!(
        cli_rust_sources_frequent_readings("src/lib.rs readme docs/notes.md", 1),
        vec![0u32]
    );
    assert_eq!(
        cli_rust_sources_frequent_readings("one/../two ./three one//end/", 2),
        vec![]
    );
    assert_eq!(
        cli_rust_sources_frequent_readings(".env report.tar.gz tmp", 3),
        vec![]
    );
    assert_eq!(
        cli_rust_sources_frequent_readings("a a/z.rs a/b.rs .cargo/config", 256),
        vec![1u32, 2u32]
    );
    assert_eq!(
        cli_rust_sources_frequent_readings("input.release output.log /tmp/ok", 0),
        vec![]
    );
    assert_eq!(
        cli_rust_sources_frequent_readings("a a/z.rs a/b.rs .cargo/config", 1),
        vec![1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2749::TRACK_PROOF, 0x52540000 | 2749);
}
