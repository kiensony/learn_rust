use task_2916::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_descending_steps(&[]), 0);
    assert_eq!(latency_improvement_descending_steps(&[4]), 0);
}
