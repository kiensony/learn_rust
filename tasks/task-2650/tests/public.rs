use task_2650::*;

#[test]
fn public_examples() {
    assert_eq!(
        embedded_nibble_swap_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[48, 96, 144, 192, 81, 240, 0, 161, 128]
        ),
        Some(9)
    );
    assert_eq!(embedded_nibble_swap_replay_mismatch(&[], &[]), None);
}
