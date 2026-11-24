use task_1617::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_distinct(&[]), vec![]);
    assert_eq!(saturation_property_distinct(&[4]), vec![0u32]);
}
