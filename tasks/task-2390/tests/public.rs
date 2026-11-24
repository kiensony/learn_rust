use task_2390::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_reverse(&[]), vec![]);
    assert_eq!(rolling_parity_reverse(&[4]), vec![4u32]);
}
