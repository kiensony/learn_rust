use task_1621::*;

#[test]
fn edge_cases() {
    assert_eq!(fastest(&[(5, 9), (5, 0)]), Some(0));
    assert_eq!(fastest(&[(5, 0), (5, 9)]), Some(0));
    assert_eq!(fastest(&[(65535, 255)]), Some(255));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1621::TRACK_PROOF, 0x52540000 | 1621);
}
