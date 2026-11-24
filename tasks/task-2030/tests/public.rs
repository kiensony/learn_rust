use task_2030::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_reverse(&[]), vec![]);
    assert_eq!(polled_groups_reverse(&[4]), vec![1u32]);
}
