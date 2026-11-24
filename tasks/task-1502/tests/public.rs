use task_1502::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_total(&[]), 0);
    assert_eq!(packet_high_nibble_total(&[4]), 0);
}
