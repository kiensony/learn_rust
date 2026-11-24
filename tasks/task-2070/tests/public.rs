use task_2070::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_reverse(&[]), vec![]);
    assert_eq!(double_buffer_receiver_reverse(&[4]), vec![4u32]);
}
