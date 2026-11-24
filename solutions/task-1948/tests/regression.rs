use task_1948::*;

#[test]
fn transfer_case() {
    assert_eq!(
        four_slot_queue_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![18u64, 27u64, 47u64, 53u64]
    );
}
