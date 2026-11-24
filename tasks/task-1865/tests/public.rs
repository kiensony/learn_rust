use task_1865::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_changes(&[]), vec![]);
    assert_eq!(weak_read_handles_changes(&[4]), vec![0u32]);
}
