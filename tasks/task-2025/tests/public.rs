use task_2025::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_changes(&[]), vec![]);
    assert_eq!(polled_groups_changes(&[4]), vec![1u32]);
}
