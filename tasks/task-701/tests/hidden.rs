use task_701::*;

#[test]
fn edge_cases() {
    assert_eq!(position(Motion::Stay, -2), -2);
    assert_eq!(position(Motion::Shift(1), 32767), 32767);
}

#[test]
fn submission_marker() {
    assert_eq!(task_701::TRACK_PROOF, 0x52540000 | 701);
}
