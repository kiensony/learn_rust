use task_1881::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_trace(&[]), vec![]);
    assert_eq!(unique_owner_state_trace(&[4]), vec![1u32]);
}
