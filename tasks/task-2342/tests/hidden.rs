use task_2342::*;

#[test]
fn edge_cases() {
    assert_eq!(baseline_drift_total(&[9, 2, 2, 8, 0, 5]), 28);
    assert_eq!(baseline_drift_total(&[0, 0, 0]), 0);
    assert_eq!(baseline_drift_total(&[255, 1, 255, 2]), 507);
    assert_eq!(
        baseline_drift_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        107
    );
    assert_eq!(baseline_drift_total(&[17, 17, 1, 0, 2]), 48);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2342::TRACK_PROOF, 0x52540000 | 2342);
}
