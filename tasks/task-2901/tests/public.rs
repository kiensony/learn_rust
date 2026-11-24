use task_2901::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_trace(&[]), vec![]);
    assert_eq!(latency_increases_trace(&[4]), vec![]);
}
