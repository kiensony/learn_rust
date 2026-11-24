use task_2918::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_steps(&[]), vec![]);
    assert_eq!(latency_improvement_steps(&[4]), vec![]);
}
