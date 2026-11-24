use task_2180::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_reverse(&[]), vec![]);
    assert_eq!(prefix_comparison_budget_reverse(&[4]), vec![0u32]);
}
