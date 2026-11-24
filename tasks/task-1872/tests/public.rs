use task_1872::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_total(&[]), 0);
    assert_eq!(shared_payload_total(&[4]), 4);
}
