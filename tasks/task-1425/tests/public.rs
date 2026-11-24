use task_1425::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_changes(&[]), vec![]);
    assert_eq!(remaining_budget_changes(&[4]), vec![18u32]);
}
