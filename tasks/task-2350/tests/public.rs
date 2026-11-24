use task_2350::*;

#[test]
fn public_examples() {
    assert_eq!(
        baseline_drift_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[0, 3, 6, 9, 18, 12, 3, 23, 5]
        ),
        Some(9)
    );
    assert_eq!(baseline_drift_replay_mismatch(&[], &[]), None);
}
