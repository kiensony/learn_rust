use task_2882::*;

#[test]
fn edge_cases() {
    assert_eq!(packed_pin_state_total(&[9, 2, 2, 8, 0, 5]), 38);
    assert_eq!(packed_pin_state_total(&[0, 0, 0]), 0);
    assert_eq!(packed_pin_state_total(&[255, 1, 255, 2]), 510);
    assert_eq!(
        packed_pin_state_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        478
    );
    assert_eq!(packed_pin_state_total(&[17, 17, 1, 0, 2]), 35);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2882::TRACK_PROOF, 0x52540000 | 2882);
}
