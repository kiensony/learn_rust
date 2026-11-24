use task_1875::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_changes(&[]), vec![]);
    assert_eq!(shared_payload_changes(&[4]), vec![4u32]);
}
