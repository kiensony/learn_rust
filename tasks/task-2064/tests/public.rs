use task_2064::*;

#[test]
fn public_examples() {
    assert_eq!(double_buffer_receiver_peaks(&[]), vec![]);
    assert_eq!(double_buffer_receiver_peaks(&[4]), vec![4u32]);
}
