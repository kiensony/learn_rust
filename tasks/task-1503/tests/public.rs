use task_1503::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_bounds(&[]), None);
    assert_eq!(packet_high_nibble_bounds(&[4]), Some((0u32, 0u32)));
}
