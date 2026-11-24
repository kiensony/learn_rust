use task_2035::*;

#[test]
fn public_examples() {
    assert_eq!(polled_masks_changes(&[]), vec![]);
    assert_eq!(polled_masks_changes(&[4]), vec![4u32]);
}
