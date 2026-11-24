use task_1382::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_total(&[]), 0);
    assert_eq!(first_frame_byte_total(&[0]), 0);
}
