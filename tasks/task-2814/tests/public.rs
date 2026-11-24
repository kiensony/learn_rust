use task_2814::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_peaks(&[]), vec![]);
    assert_eq!(service_validation_peaks(&[4]), vec![1u32]);
}
