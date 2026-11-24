use task_2594::*;

#[test]
fn edge_cases() {
    assert_eq!(
        interior_stride_proof_peaks(&[9, 2, 2, 8, 0, 5]),
        vec![3u32, 6u32]
    );
    assert_eq!(interior_stride_proof_peaks(&[0, 0, 0]), vec![1u32]);
    assert_eq!(interior_stride_proof_peaks(&[255, 1, 255, 2]), vec![0u32]);
    assert_eq!(
        interior_stride_proof_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![10u32, 16u32]
    );
    assert_eq!(interior_stride_proof_peaks(&[17, 17, 1, 0, 2]), vec![2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2594::TRACK_PROOF, 0x52540000 | 2594);
}
