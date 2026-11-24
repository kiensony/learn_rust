use task_1880::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_reverse(&[]), vec![]);
    assert_eq!(shared_payload_reverse(&[4]), vec![4u32]);
}
