use task_2399::*;

#[test]
fn public_examples() {
    assert_eq!(
        rolling_parity_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![0u32, 26u32, 3u32]
    );
    assert_eq!(rolling_parity_frequent_readings(&[], 0), vec![]);
}
