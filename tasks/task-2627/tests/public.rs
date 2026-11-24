use task_2627::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_parity_distinct(&[]), vec![]);
    assert_eq!(embedded_parity_distinct(&[4]), vec![1u32]);
}
