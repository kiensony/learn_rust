use task_583::*;

#[test]
fn edge_cases() {
    assert_eq!(heading(""), "");
    assert_eq!(heading("one\r\ntwo"), "one");
    assert_eq!(heading("\nsecond"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_583::TRACK_PROOF, 0x52540000 | 583);
}
