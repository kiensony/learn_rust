use task_1429::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_longest_run(&[]), 0);
    assert_eq!(remaining_budget_longest_run(&[4]), 1);
}
