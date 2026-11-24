use task_2965::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_changes(&[]), vec![]);
    assert_eq!(loss_accounting_changes(&[4]), vec![]);
}
