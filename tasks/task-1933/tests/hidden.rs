use task_1933::*;

#[test]
fn edge_cases() {
    assert_eq!(finish_queue(&[3, 3, 1]), Ok(vec![3, 3, 1]));
    assert_eq!(finish_queue(&[0, 255, 0]), Ok(vec![0, 255, 0]));
    assert_eq!(finish_queue(&[7]), Ok(vec![7]));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1933::TRACK_PROOF, 0x52540000 | 1933);
}
