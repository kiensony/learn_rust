use task_2600::*;

#[test]
fn public_examples() {
    assert_eq!(
        interior_stride_proof_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[10, 16]),
        Some(2)
    );
    assert_eq!(interior_stride_proof_replay_mismatch(&[], &[]), None);
}
