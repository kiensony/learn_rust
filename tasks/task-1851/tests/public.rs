use task_1851::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_trace(&[]), vec![]);
    assert_eq!(shared_read_handles_trace(&[4]), vec![1u32]);
}
