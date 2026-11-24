use task_1513::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_bounds(&[]), None);
    assert_eq!(packet_low_nibble_bounds(&[4]), Some((4u32, 4u32)));
}
