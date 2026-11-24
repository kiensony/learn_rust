use task_1932::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_total(&[]), 0);
    assert_eq!(locked_batch_totals_total(&[4]), 4);
}
