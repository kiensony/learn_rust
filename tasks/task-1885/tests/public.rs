use task_1885::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_changes(&[]), vec![]);
    assert_eq!(unique_owner_state_changes(&[4]), vec![1u32]);
}
