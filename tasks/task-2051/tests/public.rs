use task_2051::*;

#[test]
fn public_examples() {
    assert_eq!(single_slot_receiver_trace(&[]), vec![]);
    assert_eq!(single_slot_receiver_trace(&[4]), vec![4u32]);
}
