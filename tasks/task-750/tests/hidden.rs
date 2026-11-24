use task_750::*;

#[test]
fn edge_cases() {
    assert_eq!(first_ok(&[]), None);
    assert_eq!(first_ok(&[Err(()), Err(())]), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_750::TRACK_PROOF, 0x52540000 | 750);
}
