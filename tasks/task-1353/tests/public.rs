use task_1353::*;

#[test]
fn public_examples() {
    assert_eq!(frame_lengths_bounds(&[]), None);
    assert_eq!(frame_lengths_bounds(&[0]), Some((0u32, 0u32)));
}
