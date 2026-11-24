use task_2387::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_distinct(&[]), vec![]);
    assert_eq!(rolling_parity_distinct(&[4]), vec![4u32]);
}
