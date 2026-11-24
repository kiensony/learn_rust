use task_2917::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_distinct(&[]), vec![]);
    assert_eq!(latency_improvement_distinct(&[4]), vec![]);
}
