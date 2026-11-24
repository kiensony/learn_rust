use task_2150::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_occurrences_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[3, 6, 9, 12, 21, 15, 0, 26, 8]
        ),
        Some(9)
    );
    assert_eq!(first_occurrences_replay_mismatch(&[], &[]), None);
}
