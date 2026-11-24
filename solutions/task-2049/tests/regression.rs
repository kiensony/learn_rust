use task_2049::*;

#[test]
fn transfer_case() {
    assert_eq!(
        polled_masks_frequent_readings(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            2
        ),
        vec![7u32, 0u32]
    );
}
