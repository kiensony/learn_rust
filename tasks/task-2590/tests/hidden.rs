use task_2590::*;

#[test]
fn edge_cases() {
    assert_eq!(stride_proof_reverse(&[9, 2, 2, 8, 0, 5]), vec![1u32, 3u32]);
    assert_eq!(stride_proof_reverse(&[0, 0, 0]), vec![1u32]);
    assert_eq!(stride_proof_reverse(&[255, 1, 255, 2]), vec![2u32]);
    assert_eq!(
        stride_proof_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![27u32, 22u32, 7u32]
    );
    assert_eq!(stride_proof_reverse(&[17, 17, 1, 0, 2]), vec![3u32, 18u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2590::TRACK_PROOF, 0x52540000 | 2590);
}
