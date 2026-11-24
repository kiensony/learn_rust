use task_1882::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_total(&[]), 0);
    assert_eq!(unique_owner_state_total(&[4]), 1);
}
