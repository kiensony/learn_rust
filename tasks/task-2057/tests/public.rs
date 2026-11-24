use task_2057::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_distinct(&[]), vec![]);
    assert_eq!(single_slot_receiver_distinct(&[4]), vec![4u32]);
}
