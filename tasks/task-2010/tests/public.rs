use task_2010::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_reverse(&[]), vec![]);
    assert_eq!(polled_lengths_reverse(&[4]), vec![4u32]);
}
