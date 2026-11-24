use task_1510::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_reverse(&[]), vec![]);
    assert_eq!(packet_high_nibble_reverse(&[4]), vec![0u32]);
}
