use task_220::*;

#[test]
fn edge_cases() {
    assert_eq!(before_stop(&[]), 0);
    assert_eq!(before_stop(&[2, 4]), 6);
    assert_eq!(before_stop(&[0, 255, 255]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_220::TRACK_PROOF, 0x52540000 | 220);
}
