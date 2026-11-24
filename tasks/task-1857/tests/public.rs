use task_1857::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_distinct(&[]), vec![]);
    assert_eq!(shared_read_handles_distinct(&[4]), vec![1u32]);
}
