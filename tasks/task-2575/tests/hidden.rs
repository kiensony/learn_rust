use task_2575::*;

#[test]
fn edge_cases() {
    assert_eq!(
        triplet_proof_changes(&[9, 2, 2, 8, 0, 5]),
        vec![10u32, 9u32]
    );
    assert_eq!(triplet_proof_changes(&[0, 0, 0]), vec![1u32]);
    assert_eq!(triplet_proof_changes(&[255, 1, 255, 2]), vec![0u32, 3u32]);
    assert_eq!(
        triplet_proof_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![4u32, 13u32, 1u32, 32u32]
    );
    assert_eq!(triplet_proof_changes(&[17, 17, 1, 0, 2]), vec![18u32, 1u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2575::TRACK_PROOF, 0x52540000 | 2575);
}
