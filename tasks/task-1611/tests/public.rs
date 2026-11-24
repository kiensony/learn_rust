use task_1611::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_trace(&[]), vec![]);
    assert_eq!(saturation_property_trace(&[4]), vec![0u32]);
}
