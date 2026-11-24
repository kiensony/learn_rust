use task_107::*;

#[test]
fn edge_cases() {
    assert_eq!(label_size(""), 0);
    assert_eq!(label_size("   "), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_107::TRACK_PROOF, 0x52540000 | 107);
}
