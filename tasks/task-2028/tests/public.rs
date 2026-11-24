use task_2028::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_steps(&[]), vec![]);
    assert_eq!(polled_groups_steps(&[4]), vec![]);
}
