use task_2036::*;

#[test]
fn public_examples() {
    assert_eq!(polled_masks_descending_steps(&[]), 0);
    assert_eq!(polled_masks_descending_steps(&[4]), 0);
}
