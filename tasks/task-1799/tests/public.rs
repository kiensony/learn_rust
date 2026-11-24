use task_1799::*;

#[test]
fn public_examples() {
    assert_eq!(
        token_multiplier_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![12u32, 24u32, 36u32]
    );
    assert_eq!(token_multiplier_frequent_readings(&[], 0), vec![]);
}
