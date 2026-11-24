use task_2598::*;

#[test]
fn edge_cases() {
    assert_eq!(triplet_proof_rolling_load(&[4], 1), vec![5u64]);
    assert_eq!(
        triplet_proof_rolling_load(&[9, 2, 2, 8, 0, 5], 2),
        vec![19u64]
    );
    assert_eq!(triplet_proof_rolling_load(&[0, 0, 0], 0), vec![]);
    assert_eq!(triplet_proof_rolling_load(&[255, 1, 255, 2], 100), vec![]);
    assert_eq!(
        triplet_proof_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![18u64, 46u64]
    );
    assert_eq!(
        triplet_proof_rolling_load(&[17, 17, 1, 0, 2], 1),
        vec![18u64, 1u64]
    );
    assert_eq!(
        triplet_proof_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 2),
        vec![17u64, 14u64, 33u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2598::TRACK_PROOF, 0x52540000 | 2598);
}
