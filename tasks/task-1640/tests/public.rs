use task_1640::*;

#[test]
fn public_examples() {
    assert_eq!(clamp_property_reverse(&[]), vec![]);
    assert_eq!(clamp_property_reverse(&[4]), vec![0u32]);
}
