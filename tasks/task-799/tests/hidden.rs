use task_799::*;

#[test]
fn edge_cases() {
    assert_eq!(leader(&[]), None);
    assert_eq!(leader(&[255]), Some(255));
}

#[test]
fn submission_marker() {
    assert_eq!(task_799::TRACK_PROOF, 0x52540000 | 799);
}
