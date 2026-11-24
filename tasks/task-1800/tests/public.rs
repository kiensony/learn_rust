use task_1800::*;

#[test]
fn public_examples() {
    assert_eq!(
        token_offset_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[8, 11, 14, 17, 26, 20, 5, 31, 13]
        ),
        Some(9)
    );
    assert_eq!(token_offset_replay_mismatch(&[], &[]), None);
}
