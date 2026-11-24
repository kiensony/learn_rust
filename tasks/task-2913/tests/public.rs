use task_2913::*;

#[test]
fn public_examples() {
    assert_eq!(latency_improvement_bounds(&[]), None);
    assert_eq!(latency_improvement_bounds(&[4]), None);
}
