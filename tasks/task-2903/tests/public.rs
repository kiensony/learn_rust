use task_2903::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_bounds(&[]), None);
    assert_eq!(latency_increases_bounds(&[4]), None);
}
