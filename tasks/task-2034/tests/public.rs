use task_2034::*;

#[test]
fn public_examples() {
    assert_eq!(polled_masks_peaks(&[]), vec![]);
    assert_eq!(polled_masks_peaks(&[4]), vec![4u32]);
}
