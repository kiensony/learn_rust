use task_2582::*;

#[test]
fn edge_cases() {
    assert_eq!(stride_proof_total(&[9, 2, 2, 8, 0, 5]), 4);
    assert_eq!(stride_proof_total(&[0, 0, 0]), 1);
    assert_eq!(stride_proof_total(&[255, 1, 255, 2]), 2);
    assert_eq!(stride_proof_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]), 56);
    assert_eq!(stride_proof_total(&[17, 17, 1, 0, 2]), 21);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2582::TRACK_PROOF, 0x52540000 | 2582);
}
