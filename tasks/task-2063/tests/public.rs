use task_2063::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_bounds(&[]), None);
    assert_eq!(double_buffer_receiver_bounds(&[4]), Some((4u32, 4u32)));
}
