use task_2001::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_trace(&[]), vec![]);
    assert_eq!(polled_lengths_trace(&[4]), vec![4u32]);
}
