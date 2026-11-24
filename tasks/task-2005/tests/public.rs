use task_2005::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_changes(&[]), vec![]);
    assert_eq!(polled_lengths_changes(&[4]), vec![4u32]);
}
