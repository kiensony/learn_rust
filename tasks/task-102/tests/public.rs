use task_102::*;

#[test]
fn public_examples() {
    assert_eq!(opening_balance_10(65535, 4), 14);
    assert_eq!(opening_balance_10(13, 14), 23);
}
