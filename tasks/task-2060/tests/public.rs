use task_2060::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_reverse(&[]), vec![]);
    assert_eq!(single_slot_receiver_reverse(&[4]), vec![4u32]);
}
