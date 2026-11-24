use task_1383::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_bounds(&[]), None);
    assert_eq!(first_frame_byte_bounds(&[0]), Some((0u32, 0u32)));
}
