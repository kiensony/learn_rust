use task_2098::*;

#[test]
fn public_examples() {
    assert_eq!(
        three_slot_mailbox_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![3u64, 6u64, 9u64, 26u64]
    );
    assert_eq!(three_slot_mailbox_rolling_load(&[], 0), vec![]);
}
