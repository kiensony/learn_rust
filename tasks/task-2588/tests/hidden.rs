use task_2588::*;

#[test]
fn edge_cases() {
    assert_eq!(stride_proof_steps(&[9, 2, 2, 8, 0, 5]), vec![2u32]);
    assert_eq!(stride_proof_steps(&[0, 0, 0]), vec![]);
    assert_eq!(stride_proof_steps(&[255, 1, 255, 2]), vec![]);
    assert_eq!(
        stride_proof_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![15u32, 5u32]
    );
    assert_eq!(stride_proof_steps(&[17, 17, 1, 0, 2]), vec![15u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2588::TRACK_PROOF, 0x52540000 | 2588);
}
