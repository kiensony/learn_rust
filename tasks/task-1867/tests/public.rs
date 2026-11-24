use task_1867::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_distinct(&[]), vec![]);
    assert_eq!(weak_read_handles_distinct(&[4]), vec![0u32]);
}
