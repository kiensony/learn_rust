use task_2350::*;

#[test]
fn edge_cases() {
    assert_eq!(baseline_drift_replay_mismatch(&[4], &[0]), None);
    assert_eq!(
        baseline_drift_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[0, 7, 7, 1, 9, 4, 777]),
        Some(6)
    );
    assert_eq!(baseline_drift_replay_mismatch(&[0, 0, 0], &[0, 0]), Some(2));
    assert_eq!(
        baseline_drift_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        baseline_drift_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[28, 5, 23, 3, 12, 18, 9, 6, 3, 0]
        ),
        Some(0)
    );
    assert_eq!(
        baseline_drift_replay_mismatch(&[17, 17, 1, 0, 2], &[0, 0, 16, 17, 15]),
        None
    );
    assert_eq!(
        baseline_drift_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[0, 3, 6, 9, 18, 12, 3, 23, 5, 28, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2350::TRACK_PROOF, 0x52540000 | 2350);
}
