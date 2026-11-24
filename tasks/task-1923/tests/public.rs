use task_1923::*;

#[test]
fn public_examples() {
    assert_eq!(four_slot_queue_bounds(&[]), None);
    assert_eq!(four_slot_queue_bounds(&[4]), Some((4u32, 4u32)));
}
