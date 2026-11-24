use task_1428::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_steps(&[]), vec![]);
    assert_eq!(remaining_budget_steps(&[4]), vec![]);
}
