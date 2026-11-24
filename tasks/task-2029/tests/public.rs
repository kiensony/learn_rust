use task_2029::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_longest_run(&[]), 0);
    assert_eq!(polled_groups_longest_run(&[4]), 1);
}
