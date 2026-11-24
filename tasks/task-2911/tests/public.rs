use task_2911::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_trace(&[]), vec![]);
    assert_eq!(latency_improvement_trace(&[4]), vec![]);
}
