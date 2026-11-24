use task_1520::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_reverse(&[]), vec![]);
    assert_eq!(packet_low_nibble_reverse(&[4]), vec![4u32]);
}
