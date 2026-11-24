use task_1423::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_bounds(&[]), None);
    assert_eq!(remaining_budget_bounds(&[4]), Some((18u32, 18u32)));
}
