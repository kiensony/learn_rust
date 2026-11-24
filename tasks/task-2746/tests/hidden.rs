use task_2746::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_basenames_threshold_crossing("a.txt", 0), Some((0, 5)));
    assert_eq!(
        cli_basenames_threshold_crossing("src/lib.rs readme docs/notes.md", 1),
        Some((0, 6))
    );
    assert_eq!(
        cli_basenames_threshold_crossing("one/../two ./three one//end/", 5),
        Some((1, 5))
    );
    assert_eq!(
        cli_basenames_threshold_crossing(".env report.tar.gz tmp", 256),
        None
    );
    assert_eq!(
        cli_basenames_threshold_crossing("a a/z.rs a/b.rs .cargo/config", 0),
        Some((0, 1))
    );
    assert_eq!(
        cli_basenames_threshold_crossing("input.release output.log /tmp/ok", 1),
        Some((0, 13))
    );
    assert_eq!(
        cli_basenames_threshold_crossing("a a/z.rs a/b.rs .cargo/config", 7),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2746::TRACK_PROOF, 0x52540000 | 2746);
}
