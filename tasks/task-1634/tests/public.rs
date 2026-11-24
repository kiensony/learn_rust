use task_1634::*;

#[test]
fn public_examples() {
    assert_eq!(clamp_property_peaks(&[]), vec![]);
    assert_eq!(clamp_property_peaks(&[4]), vec![0u32]);
}
