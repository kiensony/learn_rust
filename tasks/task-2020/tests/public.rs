use task_2020::*;

#[test]
fn public_examples() {
    assert_eq!(polled_flags_reverse(&[]), vec![]);
    assert_eq!(polled_flags_reverse(&[4]), vec![1u32]);
}
