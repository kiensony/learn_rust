use task_1871::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_trace(&[]), vec![]);
    assert_eq!(shared_payload_trace(&[4]), vec![4u32]);
}
