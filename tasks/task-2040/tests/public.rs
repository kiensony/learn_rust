use task_2040::*;

#[test]
fn public_examples() {
    assert_eq!(polled_masks_reverse(&[]), vec![]);
    assert_eq!(polled_masks_reverse(&[4]), vec![4u32]);
}
