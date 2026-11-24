use task_2111::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_trace(&[]), vec![]);
    assert_eq!(priority_removal_trace(&[4]), vec![4u32]);
}
