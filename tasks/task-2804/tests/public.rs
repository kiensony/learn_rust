use task_2804::*;

#[test]
fn public_examples() {
    assert_eq!(service_acknowledgments_peaks(&[]), vec![]);
    assert_eq!(service_acknowledgments_peaks(&[4]), vec![4u32]);
}
