use task_210::*;

#[test]
fn edge_cases() {
    assert_eq!(knocks(0), "");
    assert_eq!(knocks(2), "!!");
    assert_eq!(knocks(7), "!!!!!!!");
}

#[test]
fn submission_marker() {
    assert_eq!(task_210::TRACK_PROOF, 0x52540000 | 210);
}
