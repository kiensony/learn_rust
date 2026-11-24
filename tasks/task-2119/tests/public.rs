use task_2119::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_longest_run(&[]), 0);
    assert_eq!(priority_removal_longest_run(&[4]), 1);
}
