use task_2962::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_total(&[]), 0);
    assert_eq!(loss_accounting_total(&[4]), 0);
}
