use task_2004::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_peaks(&[]), vec![]);
    assert_eq!(polled_lengths_peaks(&[4]), vec![4u32]);
}
