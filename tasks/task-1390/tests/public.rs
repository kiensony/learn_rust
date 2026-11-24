use task_1390::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_reverse(&[]), vec![]);
    assert_eq!(first_frame_byte_reverse(&[0]), vec![0u32]);
}
