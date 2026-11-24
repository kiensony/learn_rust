use task_1890::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_reverse(&[]), vec![]);
    assert_eq!(unique_owner_state_reverse(&[4]), vec![1u32]);
}
