use task_1876::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_descending_steps(&[]), 0);
    assert_eq!(shared_payload_descending_steps(&[4]), 0);
}
