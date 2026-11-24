use task_1381::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_trace(&[]), vec![]);
    assert_eq!(first_frame_byte_trace(&[0]), vec![0u32]);
}
