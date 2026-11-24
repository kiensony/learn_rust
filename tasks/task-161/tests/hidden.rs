use task_161::*;

#[test]
fn edge_cases() {
    assert_eq!(door(true), "locked");
    assert_eq!(door(false), "open");
}

#[test]
fn submission_marker() {
    assert_eq!(task_161::TRACK_PROOF, 0x52540000 | 161);
}
