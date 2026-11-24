use task_2011::*;

#[test]
fn public_examples() {
    assert_eq!(polled_flags_trace(&[]), vec![]);
    assert_eq!(polled_flags_trace(&[4]), vec![1u32]);
}
