use task_2888::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packed_pin_state_steps(&[9, 2, 2, 8, 0, 5]),
        vec![34u32, 2u32]
    );
    assert_eq!(packed_pin_state_steps(&[0, 0, 0]), vec![]);
    assert_eq!(packed_pin_state_steps(&[255, 1, 255, 2]), vec![0u32]);
    assert_eq!(
        packed_pin_state_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![48u32, 6u32, 138u32, 4u32]
    );
    assert_eq!(packed_pin_state_steps(&[17, 17, 1, 0, 2]), vec![33u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2888::TRACK_PROOF, 0x52540000 | 2888);
}
