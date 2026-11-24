use task_2172::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_total(&[]), 0);
    assert_eq!(prefix_comparison_budget_total(&[4]), 0);
}
