use task_2750::*;

#[test]
fn edge_cases() {
    assert_eq!(cli_stems_replay_mismatch("a.txt", &[1]), None);
    assert_eq!(
        cli_stems_replay_mismatch("src/lib.rs readme docs/notes.md", &[3, 6, 5, 777]),
        Some(3)
    );
    assert_eq!(
        cli_stems_replay_mismatch("one/../two ./three one//end/", &[3, 5]),
        Some(2)
    );
    assert_eq!(
        cli_stems_replay_mismatch(".env report.tar.gz tmp", &[9999]),
        Some(0)
    );
    assert_eq!(
        cli_stems_replay_mismatch("a a/z.rs a/b.rs .cargo/config", &[6, 1, 1, 1]),
        Some(0)
    );
    assert_eq!(
        cli_stems_replay_mismatch("input.release output.log /tmp/ok", &[5, 6, 2]),
        None
    );
    assert_eq!(
        cli_stems_replay_mismatch("src/lib.rs readme docs/notes.md", &[3, 6, 5, 1]),
        Some(3)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2750::TRACK_PROOF, 0x52540000 | 2750);
}
