use task_1878::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_steps(&[]), vec![]);
    assert_eq!(shared_payload_steps(&[4]), vec![]);
}
