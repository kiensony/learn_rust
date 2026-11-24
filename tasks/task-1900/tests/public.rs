use task_1900::*;

#[test]
fn public_examples() {
    assert_eq!(
        weak_liveness_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[1, 0, 1, 0, 1, 1, 0, 0, 0]
        ),
        Some(9)
    );
    assert_eq!(weak_liveness_replay_mismatch(&[], &[]), None);
}
