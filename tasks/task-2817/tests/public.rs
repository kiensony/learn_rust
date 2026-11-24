use task_2817::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_distinct(&[]), vec![]);
    assert_eq!(service_validation_distinct(&[4]), vec![1u32]);
}
