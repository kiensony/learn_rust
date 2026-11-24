use task_1614::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_peaks(&[]), vec![]);
    assert_eq!(saturation_property_peaks(&[4]), vec![0u32]);
}
