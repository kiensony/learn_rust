use task_2027::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_distinct(&[]), vec![]);
    assert_eq!(polled_groups_distinct(&[4]), vec![1u32]);
}
