use task_2699::*;

#[test]
fn transfer_case() {
    assert_eq!(
        universal_slice_sum_frequent_readings(&[9, 2, 2, 8, 0, 5, 5, 0, 8, 2, 2, 9], 2),
        vec![21u32, 20u32]
    );
}
