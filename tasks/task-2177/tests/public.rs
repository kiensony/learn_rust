use task_2177::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_distinct(&[]), vec![]);
    assert_eq!(prefix_comparison_budget_distinct(&[4]), vec![0u32]);
}
