use task_2067::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_distinct(&[]), vec![]);
    assert_eq!(double_buffer_receiver_distinct(&[4]), vec![4u32]);
}
