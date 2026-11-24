use task_1937::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_distinct(&[]), vec![]);
    assert_eq!(locked_batch_totals_distinct(&[4]), vec![4u32]);
}
