use task_2630::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_parity_reverse(&[]), vec![]);
    assert_eq!(embedded_parity_reverse(&[4]), vec![1u32]);
}
