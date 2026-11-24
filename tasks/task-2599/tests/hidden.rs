use task_2599::*;

#[test]
fn edge_cases() {
    assert_eq!(stride_proof_frequent_readings(&[4], 0), vec![]);
    assert_eq!(
        stride_proof_frequent_readings(&[9, 2, 2, 8, 0, 5], 1),
        vec![3u32]
    );
    assert_eq!(stride_proof_frequent_readings(&[0, 0, 0], 2), vec![1u32]);
    assert_eq!(
        stride_proof_frequent_readings(&[255, 1, 255, 2], 3),
        vec![2u32]
    );
    assert_eq!(
        stride_proof_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 256),
        vec![7u32, 22u32, 27u32]
    );
    assert_eq!(
        stride_proof_frequent_readings(&[17, 17, 1, 0, 2], 0),
        vec![]
    );
    assert_eq!(
        stride_proof_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![7u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2599::TRACK_PROOF, 0x52540000 | 2599);
}
