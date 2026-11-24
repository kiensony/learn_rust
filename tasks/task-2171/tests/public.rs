use task_2171::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_trace(&[]), vec![]);
    assert_eq!(prefix_comparison_budget_trace(&[4]), vec![0u32]);
}
