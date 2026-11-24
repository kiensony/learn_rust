use task_1620::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_reverse(&[]), vec![]);
    assert_eq!(saturation_property_reverse(&[4]), vec![0u32]);
}
