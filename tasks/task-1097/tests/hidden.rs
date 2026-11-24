use task_1097::*;

#[test]
fn edge_cases() {
    assert_eq!(total_span(&[] as &[Length]), 0);
    assert_eq!(total_span(&[Length(u32::MAX), Length(1)]), 4294967296);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1097::TRACK_PROOF, 0x52540000 | 1097);
}
