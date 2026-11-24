use task_2014::*;

#[test]
fn public_examples() {
    assert_eq!(polled_flags_peaks(&[]), vec![]);
    assert_eq!(polled_flags_peaks(&[4]), vec![1u32]);
}
