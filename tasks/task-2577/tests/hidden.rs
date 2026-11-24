use task_2577::*;

#[test]
fn edge_cases() {
    assert_eq!(
        triplet_proof_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![9u32, 10u32]
    );
    assert_eq!(triplet_proof_distinct(&[0, 0, 0]), vec![1u32]);
    assert_eq!(triplet_proof_distinct(&[255, 1, 255, 2]), vec![0u32, 3u32]);
    assert_eq!(
        triplet_proof_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![1u32, 4u32, 13u32, 32u32]
    );
    assert_eq!(
        triplet_proof_distinct(&[17, 17, 1, 0, 2]),
        vec![1u32, 18u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2577::TRACK_PROOF, 0x52540000 | 2577);
}
