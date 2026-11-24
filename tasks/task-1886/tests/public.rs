use task_1886::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_descending_steps(&[]), 0);
    assert_eq!(unique_owner_state_descending_steps(&[4]), 0);
}
