use task_962::*;

#[test]
fn edge_cases() {
    assert_eq!(legs(&[]), vec![]);
    assert_eq!(legs(&[0]), vec![]);
    assert_eq!(legs(&[255, 0, 255]), vec![(255, 0), (0, 255)]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_962::TRACK_PROOF, 0x52540000 | 962);
}
