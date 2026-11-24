use task_1644::*;

#[test]
fn public_examples() {
    assert_eq!(shift_property_peaks(&[]), vec![]);
    assert_eq!(shift_property_peaks(&[4]), vec![0u32]);
}
