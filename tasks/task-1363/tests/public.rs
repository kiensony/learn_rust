use task_1363::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_bounds(&[]), None);
    assert_eq!(frame_offsets_bounds(&[0]), Some((0u32, 0u32)));
}
