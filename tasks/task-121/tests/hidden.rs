use task_121::*;

#[test]
fn edge_cases() {
    assert_eq!(as_byte(0), Some(0));
    assert_eq!(as_byte(-1), None);
    assert_eq!(as_byte(256), None);
    assert_eq!(as_byte(-32768), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_121::TRACK_PROOF, 0x52540000 | 121);
}
