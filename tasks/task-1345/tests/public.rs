use task_1345::*;

#[test]
fn public_examples() {
    assert_eq!(packed_booleans_changes(""), vec![]);
    assert_eq!(packed_booleans_changes("1"), vec![1u32]);
}
