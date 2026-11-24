use task_2966::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_descending_steps(&[]), 0);
    assert_eq!(loss_accounting_descending_steps(&[4]), 0);
}
