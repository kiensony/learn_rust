use task_2038::*;

#[test]
fn public_examples() {
    assert_eq!(polled_masks_steps(&[]), vec![]);
    assert_eq!(polled_masks_steps(&[4]), vec![]);
}
