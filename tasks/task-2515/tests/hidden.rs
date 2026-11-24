use task_2515::*;

#[test]
fn edge_cases() {
    assert_eq!(scaled_reading(i16::MIN, 255), -8355840);
    assert_eq!(scaled_reading(i16::MAX, 0), 0);
    assert_eq!(scaled_reading(256, 1), 256);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2515::TRACK_PROOF, 0x52540000 | 2515);
}
