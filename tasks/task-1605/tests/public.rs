use task_1605::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_changes(&[]), vec![]);
    assert_eq!(parity_property_changes(&[4]), vec![0u32]);
}
