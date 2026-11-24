use task_2049::*;

#[test]
fn public_examples() {
    assert_eq!(
        polled_masks_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![7u32, 0u32, 3u32]
    );
    assert_eq!(polled_masks_frequent_readings(&[], 0), vec![]);
}
