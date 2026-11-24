use task_2000::*;

#[test]
fn public_examples() {
    assert_eq!(
        atomic_exchange_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[0, 3, 6, 9, 12, 21, 15, 0, 26]
        ),
        Some(9)
    );
    assert_eq!(atomic_exchange_replay_mismatch(&[], &[]), None);
}
