use task_1422::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_total(&[]), 0);
    assert_eq!(remaining_budget_total(&[4]), 18);
}
