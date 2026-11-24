use task_603::*;

#[test]
fn edge_cases() {
    assert_eq!(choose(true, "", "ok"), "");
    assert_eq!(choose(false, "urgent", ""), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_603::TRACK_PROOF, 0x52540000 | 603);
}
