use task_2902::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_total(&[]), 0);
    assert_eq!(latency_increases_total(&[4]), 0);
}
