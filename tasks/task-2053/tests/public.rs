use task_2053::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_bounds(&[]), None);
    assert_eq!(single_slot_receiver_bounds(&[4]), Some((4u32, 4u32)));
}
