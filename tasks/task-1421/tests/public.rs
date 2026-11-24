use task_1421::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_trace(&[]), vec![]);
    assert_eq!(remaining_budget_trace(&[4]), vec![18u32]);
}
