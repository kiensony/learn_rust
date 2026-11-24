use task_2820::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_reverse(&[]), vec![]);
    assert_eq!(service_validation_reverse(&[4]), vec![1u32]);
}
