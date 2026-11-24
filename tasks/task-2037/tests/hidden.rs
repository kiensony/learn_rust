use task_2037::*;

#[test]
fn edge_cases() {
    assert_eq!(poll_gate(2), (3, 2));
    assert_eq!(poll_gate(13), (14, 13));
    assert_eq!(poll_gate(255), (256, 255));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2037::TRACK_PROOF, 0x52540000 | 2037);
}
