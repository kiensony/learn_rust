use task_260::*;

#[test]
fn edge_cases() {
    assert_eq!(width(""), 0);
    assert_eq!(width("猫"), 3);
    assert_eq!(width("a b"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_260::TRACK_PROOF, 0x52540000 | 260);
}
