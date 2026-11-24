use task_156::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_debit_10(65535, 4), 0);
    assert_eq!(saturating_debit_10(13, 14), 3);
}
