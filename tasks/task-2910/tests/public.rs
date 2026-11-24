use task_2910::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_reverse(&[]), vec![]);
    assert_eq!(latency_increases_reverse(&[4]), vec![]);
}
