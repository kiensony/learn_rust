use task_146::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_debit_8(4, 0), 0);
    assert_eq!(saturating_debit_8(13, 1), 4);
}
