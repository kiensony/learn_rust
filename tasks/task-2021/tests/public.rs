use task_2021::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_trace(&[]), vec![]);
    assert_eq!(polled_groups_trace(&[4]), vec![1u32]);
}
