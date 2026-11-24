use task_2174::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_peaks(&[]), vec![]);
    assert_eq!(prefix_comparison_budget_peaks(&[4]), vec![0u32]);
}
