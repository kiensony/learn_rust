use task_2499::*;

#[test]
fn public_examples() {
    assert_eq!(
        trailing_raw_reads_frequent_readings(&[9, 2, 2, 8, 0, 5], 3),
        vec![2u32, 8u32, 0u32]
    );
    assert_eq!(trailing_raw_reads_frequent_readings(&[], 0), vec![]);
}
