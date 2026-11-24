use task_2699::*;

#[test]
fn public_examples() {
    assert_eq!(
        universal_slice_sum_frequent_readings(&[9, 2, 2, 8, 0, 5], 3),
        vec![21u32, 5u32]
    );
    assert_eq!(universal_slice_sum_frequent_readings(&[], 0), vec![]);
}
