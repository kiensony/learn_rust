use task_1618::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_steps(&[]), vec![]);
    assert_eq!(saturation_property_steps(&[4]), vec![]);
}
