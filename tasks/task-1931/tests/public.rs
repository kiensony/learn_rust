use task_1931::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_trace(&[]), vec![]);
    assert_eq!(locked_batch_totals_trace(&[4]), vec![4u32]);
}
