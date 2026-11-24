use task_949::*;

#[test]
fn edge_cases() {
    assert_eq!(rises(&[]), vec![]);
    assert_eq!(rises(&[2]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_949::TRACK_PROOF, 0x52540000 | 949);
}
