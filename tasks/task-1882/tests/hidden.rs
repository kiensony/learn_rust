use task_1882::*;

#[test]
fn edge_cases() {
    assert_eq!(unique_owner_state_total(&[9, 2, 2, 8, 0, 5]), 2);
    assert_eq!(unique_owner_state_total(&[0, 0, 0]), 3);
    assert_eq!(unique_owner_state_total(&[255, 1, 255, 2]), 0);
    assert_eq!(
        unique_owner_state_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        3
    );
    assert_eq!(unique_owner_state_total(&[17, 17, 1, 0, 2]), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1882::TRACK_PROOF, 0x52540000 | 1882);
}
