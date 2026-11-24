use task_2065::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_changes(&[]), vec![]);
    assert_eq!(double_buffer_receiver_changes(&[4]), vec![4u32]);
}
