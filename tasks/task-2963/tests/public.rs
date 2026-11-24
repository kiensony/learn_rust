use task_2963::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_bounds(&[]), None);
    assert_eq!(loss_accounting_bounds(&[4]), None);
}
