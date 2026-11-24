use task_1870::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_reverse(&[]), vec![]);
    assert_eq!(weak_read_handles_reverse(&[4]), vec![0u32]);
}
