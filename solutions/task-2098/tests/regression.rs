use task_2098::*;

#[test]
fn transfer_case() {
    assert_eq!(
        three_slot_mailbox_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![18u64, 41u64, 50u64]
    );
}
