use task_2045::*;

#[test]
fn public_examples() {
    assert_eq!(polled_costs_changes(&[]), vec![]);
    assert_eq!(polled_costs_changes(&[4]), vec![1u32]);
}
