use task_2881::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packed_pin_state_trace(&[9, 2, 2, 8, 0, 5]),
        vec![36u32, 2u32, 0u32]
    );
    assert_eq!(packed_pin_state_trace(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        packed_pin_state_trace(&[255, 1, 255, 2]),
        vec![255u32, 255u32]
    );
    assert_eq!(
        packed_pin_state_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![192u32, 144u32, 138u32, 0u32, 4u32]
    );
    assert_eq!(
        packed_pin_state_trace(&[17, 17, 1, 0, 2]),
        vec![34u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2881::TRACK_PROOF, 0x52540000 | 2881);
}
