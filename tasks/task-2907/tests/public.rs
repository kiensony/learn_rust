use task_2907::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_distinct(&[]), vec![]);
    assert_eq!(latency_increases_distinct(&[4]), vec![]);
}
