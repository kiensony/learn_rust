use task_2173::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_comparison_budget_bounds(&[]), None);
    assert_eq!(prefix_comparison_budget_bounds(&[4]), Some((0u32, 0u32)));
}
