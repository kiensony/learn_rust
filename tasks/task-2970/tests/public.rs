use task_2970::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_reverse(&[]), vec![]);
    assert_eq!(loss_accounting_reverse(&[4]), vec![]);
}
