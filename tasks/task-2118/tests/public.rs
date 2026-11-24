use task_2118::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_steps(&[]), vec![]);
    assert_eq!(priority_removal_steps(&[4]), vec![]);
}
