use task_1366::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_descending_steps(&[]), 0);
    assert_eq!(frame_offsets_descending_steps(&[0]), 0);
}
