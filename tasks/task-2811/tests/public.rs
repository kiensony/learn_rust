use task_2811::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_trace(&[]), vec![]);
    assert_eq!(service_validation_trace(&[4]), vec![1u32]);
}
