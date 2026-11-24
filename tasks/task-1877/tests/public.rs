use task_1877::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_distinct(&[]), vec![]);
    assert_eq!(shared_payload_distinct(&[4]), vec![4u32]);
}
