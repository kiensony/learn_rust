use task_2400::*;

#[test]
fn public_examples() {
    assert_eq!(
        last_change_age_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[1, 1, 1, 1, 1, 1, 1, 1, 1]
        ),
        Some(9)
    );
    assert_eq!(last_change_age_replay_mismatch(&[], &[]), None);
}
