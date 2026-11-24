use task_2625::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_parity_changes(&[]), vec![]);
    assert_eq!(embedded_parity_changes(&[4]), vec![1u32]);
}
