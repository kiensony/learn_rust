use task_2598::*;

#[test]
fn public_examples() {
    assert_eq!(
        triplet_proof_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![4u64, 13u64, 1u64, 32u64]
    );
    assert_eq!(triplet_proof_rolling_load(&[], 0), vec![]);
}
