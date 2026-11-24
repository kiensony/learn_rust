use task_2115::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_changes(&[]), vec![]);
    assert_eq!(priority_removal_changes(&[4]), vec![4u32]);
}
