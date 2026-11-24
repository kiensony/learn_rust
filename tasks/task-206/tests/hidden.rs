use task_206::*;

#[test]
fn edge_cases() {
    assert_eq!(admitted(false, true, false), true);
    assert_eq!(admitted(false, false, false), false);
    assert_eq!(admitted(false, false, true), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_206::TRACK_PROOF, 0x52540000 | 206);
}
