use task_1889::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_longest_run(&[]), 0);
    assert_eq!(unique_owner_state_longest_run(&[4]), 1);
}
