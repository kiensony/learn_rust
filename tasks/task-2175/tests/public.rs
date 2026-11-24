use task_2175::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_changes(&[]), vec![]);
    assert_eq!(prefix_comparison_budget_changes(&[4]), vec![0u32]);
}
