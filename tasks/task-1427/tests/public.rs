use task_1427::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_distinct(&[]), vec![]);
    assert_eq!(remaining_budget_distinct(&[4]), vec![18u32]);
}
