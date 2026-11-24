use task_2349::*;

#[test]
fn public_examples() {
    assert_eq!(
        batch_variation_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![9u32, 26u32, 23u32]
    );
    assert_eq!(batch_variation_frequent_readings(&[], 0), vec![]);
}
