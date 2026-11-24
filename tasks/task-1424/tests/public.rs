use task_1424::*;

#[test]
fn public_examples() {
    assert_eq!(remaining_budget_peaks(&[]), vec![]);
    assert_eq!(remaining_budget_peaks(&[4]), vec![18u32]);
}
