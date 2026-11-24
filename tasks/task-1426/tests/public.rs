use task_1426::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_descending_steps(&[]), 0);
    assert_eq!(remaining_budget_descending_steps(&[4]), 0);
}
