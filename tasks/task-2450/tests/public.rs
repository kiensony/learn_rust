use task_2450::*;

#[test]
fn public_examples() {
    assert_eq!(
        page_alignment_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[32, 32, 32, 32, 32, 32, 0, 32, 32]
        ),
        Some(9)
    );
    assert_eq!(page_alignment_replay_mismatch(&[], &[]), None);
}
