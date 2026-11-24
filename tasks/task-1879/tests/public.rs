use task_1879::*;

#[test]
fn public_examples() {
    assert_eq!(shared_payload_longest_run(&[]), 0);
    assert_eq!(shared_payload_longest_run(&[4]), 1);
}
