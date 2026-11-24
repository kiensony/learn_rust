use task_2750::*;

#[test]
fn public_examples() {
    assert_eq!(
        cli_stems_replay_mismatch("src/lib.rs readme docs/notes.md", &[3, 6]),
        Some(2)
    );
    assert_eq!(cli_stems_replay_mismatch("", &[]), None);
}
