use task_2981::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_trace(&[]), vec![]);
    assert_eq!(service_wait_trace(&[4]), vec![]);
}
