use task_1613::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_bounds(&[]), None);
    assert_eq!(saturation_property_bounds(&[4]), Some((0u32, 0u32)));
}
