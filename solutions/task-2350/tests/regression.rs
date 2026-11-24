use task_2350::*;

#[test]
fn transfer_case() {
    assert_eq!(
        baseline_drift_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[
                0, 3, 6, 9, 18, 12, 3, 23, 5, 28, 28, 5, 23, 3, 12, 18, 9, 6, 3, 0
            ]
        ),
        None
    );
}
