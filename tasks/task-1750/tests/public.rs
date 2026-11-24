use task_1750::*;

#[test]
fn public_examples() {
    assert_eq!(
        rotate_macro_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[12, 24, 36, 48, 84, 60, 0, 104, 32]
        ),
        Some(9)
    );
    assert_eq!(rotate_macro_replay_mismatch(&[], &[]), None);
}
