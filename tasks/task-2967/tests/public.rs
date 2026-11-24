use task_2967::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_distinct(&[]), vec![]);
    assert_eq!(loss_accounting_distinct(&[4]), vec![]);
}
