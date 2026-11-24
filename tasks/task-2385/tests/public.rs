use task_2385::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_changes(&[]), vec![]);
    assert_eq!(rolling_parity_changes(&[4]), vec![4u32]);
}
