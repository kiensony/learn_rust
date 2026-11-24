use task_2592::*;

#[test]
fn edge_cases() {
    assert_eq!(interior_stride_proof_total(&[9, 2, 2, 8, 0, 5]), 9);
    assert_eq!(interior_stride_proof_total(&[0, 0, 0]), 1);
    assert_eq!(interior_stride_proof_total(&[255, 1, 255, 2]), 0);
    assert_eq!(
        interior_stride_proof_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        35
    );
    assert_eq!(interior_stride_proof_total(&[17, 17, 1, 0, 2]), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2592::TRACK_PROOF, 0x52540000 | 2592);
}
