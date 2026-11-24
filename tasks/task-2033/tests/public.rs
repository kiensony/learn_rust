use task_2033::*;

#[test]
fn public_examples() {
    assert_eq!(polled_masks_bounds(&[]), None);
    assert_eq!(polled_masks_bounds(&[4]), Some((4u32, 4u32)));
}
