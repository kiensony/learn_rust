use task_1610::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_reverse(&[]), vec![]);
    assert_eq!(parity_property_reverse(&[4]), vec![0u32]);
}
