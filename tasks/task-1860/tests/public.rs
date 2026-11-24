use task_1860::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_reverse(&[]), vec![]);
    assert_eq!(shared_read_handles_reverse(&[4]), vec![1u32]);
}
