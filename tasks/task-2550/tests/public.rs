use task_2550::*;

#[test]
fn public_examples() {
    assert_eq!(
        abi_status_mask_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[3, 6, 9, 12, 21, 15, 0, 26, 8]
        ),
        Some(9)
    );
    assert_eq!(abi_status_mask_replay_mismatch(&[], &[]), None);
}
