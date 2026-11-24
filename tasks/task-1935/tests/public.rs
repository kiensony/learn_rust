use task_1935::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_changes(&[]), vec![]);
    assert_eq!(locked_batch_totals_changes(&[4]), vec![4u32]);
}
