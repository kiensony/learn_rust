use task_2061::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_trace(&[]), vec![]);
    assert_eq!(double_buffer_receiver_trace(&[4]), vec![4u32]);
}
