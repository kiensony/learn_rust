use task_2964::*;

#[test]
fn public_examples() {
    assert_eq!(loss_accounting_peaks(&[]), vec![]);
    assert_eq!(loss_accounting_peaks(&[4]), vec![]);
}
