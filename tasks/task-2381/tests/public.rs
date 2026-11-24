use task_2381::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_trace(&[]), vec![]);
    assert_eq!(rolling_parity_trace(&[4]), vec![4u32]);
}
