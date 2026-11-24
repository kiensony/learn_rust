use task_1938::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_steps(&[]), vec![]);
    assert_eq!(locked_batch_totals_steps(&[4]), vec![]);
}
