use task_1936::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_descending_steps(&[]), 0);
    assert_eq!(locked_batch_totals_descending_steps(&[4]), 0);
}
