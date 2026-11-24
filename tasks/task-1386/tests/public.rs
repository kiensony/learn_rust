use task_1386::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_descending_steps(&[]), 0);
    assert_eq!(first_frame_byte_descending_steps(&[0]), 0);
}
