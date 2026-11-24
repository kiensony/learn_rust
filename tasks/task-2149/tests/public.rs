use task_2149::*;

#[test]
fn public_examples() {
    assert_eq!(
        histogram_bins_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![1u32]
    );
    assert_eq!(histogram_bins_frequent_readings(&[], 0), vec![]);
}
