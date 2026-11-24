use task_1373::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_bounds(&[]), None);
    assert_eq!(frame_checksum_bounds(&[0]), Some((0u32, 0u32)));
}
