use task_1887::*;

#[test]
fn public_examples() {
    assert_eq!(unique_owner_state_distinct(&[]), vec![]);
    assert_eq!(unique_owner_state_distinct(&[4]), vec![1u32]);
}
