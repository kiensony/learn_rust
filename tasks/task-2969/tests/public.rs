use task_2969::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_longest_run(&[]), 0);
    assert_eq!(loss_accounting_longest_run(&[4]), 0);
}
