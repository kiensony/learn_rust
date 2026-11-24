use task_1861::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_trace(&[]), vec![]);
    assert_eq!(weak_read_handles_trace(&[4]), vec![0u32]);
}
