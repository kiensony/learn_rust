use task_1888::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_steps(&[]), vec![]);
    assert_eq!(unique_owner_state_steps(&[4]), vec![]);
}
