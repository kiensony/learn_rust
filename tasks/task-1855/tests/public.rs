use task_1855::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_changes(&[]), vec![]);
    assert_eq!(shared_read_handles_changes(&[4]), vec![1u32]);
}
