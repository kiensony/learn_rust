use task_2041::*;

#[test]
fn public_examples() {
    assert_eq!(polled_costs_trace(&[]), vec![]);
    assert_eq!(polled_costs_trace(&[4]), vec![1u32]);
}
