use task_2499::*;

#[test]
fn transfer_case() {
    assert_eq!(
        trailing_raw_reads_frequent_readings(&[9, 2, 2, 8, 0, 5, 5, 0, 8, 2, 2, 9], 2),
        vec![2u32, 8u32]
    );
}
