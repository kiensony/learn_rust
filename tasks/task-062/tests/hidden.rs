use task_062::*;

#[test]
fn edge_cases() {
    assert_eq!(open_sign(false), false);
    assert_eq!(open_sign(true), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_062::TRACK_PROOF, 0x52540000 | 62);
}
