use task_1949::*;

#[test]
fn public_examples() {
    assert_eq!(
        locked_batch_totals_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![9u32, 21u32, 36u32]
    );
    assert_eq!(locked_batch_totals_frequent_readings(&[], 0), vec![]);
}
