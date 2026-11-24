use task_978::*;

#[test]
fn edge_cases() {
    assert_eq!(negative_at(&[]), None);
    assert_eq!(negative_at(&[-1]), Some(0));
    assert_eq!(negative_at(&[-1, 6, -8]), Some(0));
}

#[test]
fn submission_marker() {
    assert_eq!(task_978::TRACK_PROOF, 0x52540000 | 978);
}
