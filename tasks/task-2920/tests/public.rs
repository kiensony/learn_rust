use task_2920::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_reverse(&[]), vec![]);
    assert_eq!(latency_improvement_reverse(&[4]), vec![]);
}
