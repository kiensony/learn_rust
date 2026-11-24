use task_137::*;

#[test]
fn edge_cases() {
    assert_eq!(midpoint(-4, -1), -2.5);
    assert_eq!(midpoint(-32768, 32767), -0.5);
    assert_eq!(midpoint(4, 4), 4.0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_137::TRACK_PROOF, 0x52540000 | 137);
}
