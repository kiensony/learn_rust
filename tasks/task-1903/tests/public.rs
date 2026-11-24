use task_1903::*;

#[test]
fn public_examples() {
    assert_eq!(one_slot_queue_bounds(&[]), None);
    assert_eq!(one_slot_queue_bounds(&[4]), Some((4u32, 4u32)));
}
