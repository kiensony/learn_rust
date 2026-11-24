use task_2062::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_total(&[]), 0);
    assert_eq!(double_buffer_receiver_total(&[4]), 4);
}
