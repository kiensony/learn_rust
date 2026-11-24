use task_2915::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_changes(&[]), vec![]);
    assert_eq!(latency_improvement_changes(&[4]), vec![]);
}
