use task_779::*;

#[test]
fn edge_cases() {
    assert_eq!(unique(&[&[1], &[2]]), 2);
    assert_eq!(unique(&[&[], &[]]), 0);
    assert_eq!(unique(&[&[0, 255, 0]]), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_779::TRACK_PROOF, 0x52540000 | 779);
}
