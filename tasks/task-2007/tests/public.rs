use task_2007::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_distinct(&[]), vec![]);
    assert_eq!(polled_lengths_distinct(&[4]), vec![4u32]);
}
