use task_2600::*;

#[test]
fn transfer_case() {
    assert_eq!(
        interior_stride_proof_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[10, 16, 9, 9, 16, 10]
        ),
        None
    );
}
