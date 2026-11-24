use task_2996::*;

#[test]
fn edge_cases() {
    assert_eq!(decode_batch(&[3, 9]), (vec![], vec![3, 9]));
    assert_eq!(decode_batch(&[1, 7, 2]), (vec![vec![7]], vec![2]));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2996::TRACK_PROOF, 0x52540000 | 2996);
}
