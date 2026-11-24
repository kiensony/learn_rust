use task_2750::*;

#[test]
fn transfer_case() {
    assert_eq!(
        cli_stems_replay_mismatch(
            "src/lib.rs readme docs/notes.md src/lib.rs readme docs/notes.md",
            &[3, 6, 5, 3, 6, 5]
        ),
        None
    );
}
