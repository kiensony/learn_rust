use task_1612::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_total(&[]), 0);
    assert_eq!(saturation_property_total(&[4]), 0);
}
