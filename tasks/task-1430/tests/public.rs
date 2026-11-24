use task_1430::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_reverse(&[]), vec![]);
    assert_eq!(remaining_budget_reverse(&[4]), vec![18u32]);
}
