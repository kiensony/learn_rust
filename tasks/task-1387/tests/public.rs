use task_1387::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_distinct(&[]), vec![]);
    assert_eq!(first_frame_byte_distinct(&[0]), vec![0u32]);
}
