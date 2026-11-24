use task_1385::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_changes(&[]), vec![]);
    assert_eq!(first_frame_byte_changes(&[0]), vec![0u32]);
}
