use task_1362::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_total(&[]), 0);
    assert_eq!(frame_offsets_total(&[0]), 0);
}
