use task_2032::*;

#[test]
fn public_examples() {
    assert_eq!(polled_masks_total(&[]), 0);
    assert_eq!(polled_masks_total(&[4]), 4);
}
