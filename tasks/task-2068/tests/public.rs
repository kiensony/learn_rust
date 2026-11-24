use task_2068::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_steps(&[]), vec![]);
    assert_eq!(double_buffer_receiver_steps(&[4]), vec![]);
}
