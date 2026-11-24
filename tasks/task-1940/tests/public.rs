use task_1940::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_reverse(&[]), vec![]);
    assert_eq!(locked_batch_totals_reverse(&[4]), vec![4u32]);
}
