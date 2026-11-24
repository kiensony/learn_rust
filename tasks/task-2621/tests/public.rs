use task_2621::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_parity_trace(&[]), vec![]);
    assert_eq!(embedded_parity_trace(&[4]), vec![1u32]);
}
