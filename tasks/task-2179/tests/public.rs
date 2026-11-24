use task_2179::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_longest_run(&[]), 0);
    assert_eq!(prefix_comparison_budget_longest_run(&[4]), 1);
}
