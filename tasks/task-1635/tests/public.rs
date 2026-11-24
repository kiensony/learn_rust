use task_1635::*;

#[test]
fn public_examples() {
    assert_eq!(clamp_property_changes(&[]), vec![]);
    assert_eq!(clamp_property_changes(&[4]), vec![0u32]);
}
