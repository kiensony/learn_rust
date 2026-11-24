use task_1127::*;

#[test]
fn edge_cases() {
    assert_eq!(byte_label(vec![]), "0 bytes");
    assert_eq!(byte_label(vec![0, 255, 0]), "3 bytes");
    assert_eq!(byte_label(vec![9, 9, 9, 9, 9, 9, 9, 9]), "8 bytes");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1127::TRACK_PROOF, 0x52540000 | 1127);
}
