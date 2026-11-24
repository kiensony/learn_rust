use task_2841::*;

#[test]
fn public_examples() {
    assert_eq!(service_status_trace(&[]), vec![]);
    assert_eq!(service_status_trace(&[4]), vec![0u32]);
}
