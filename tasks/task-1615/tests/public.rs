use task_1615::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_changes(&[]), vec![]);
    assert_eq!(saturation_property_changes(&[4]), vec![0u32]);
}
