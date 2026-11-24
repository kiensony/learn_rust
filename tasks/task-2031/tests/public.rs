use task_2031::*;

#[test]
fn public_examples() {
    assert_eq!(polled_masks_trace(&[]), vec![]);
    assert_eq!(polled_masks_trace(&[4]), vec![4u32]);
}
