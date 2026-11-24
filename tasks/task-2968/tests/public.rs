use task_2968::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_steps(&[]), vec![]);
    assert_eq!(loss_accounting_steps(&[4]), vec![]);
}
