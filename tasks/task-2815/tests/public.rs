use task_2815::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_changes(&[]), vec![]);
    assert_eq!(service_validation_changes(&[4]), vec![1u32]);
}
