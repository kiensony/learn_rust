use task_2178::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_steps(&[]), vec![]);
    assert_eq!(prefix_comparison_budget_steps(&[4]), vec![]);
}
