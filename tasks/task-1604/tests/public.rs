use task_1604::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_peaks(&[]), vec![]);
    assert_eq!(parity_property_peaks(&[4]), vec![0u32]);
}
