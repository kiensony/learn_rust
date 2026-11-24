use task_1388::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_steps(&[]), vec![]);
    assert_eq!(first_frame_byte_steps(&[0]), vec![]);
}
