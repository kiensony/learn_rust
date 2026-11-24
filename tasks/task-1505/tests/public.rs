use task_1505::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_changes(&[]), vec![]);
    assert_eq!(packet_high_nibble_changes(&[4]), vec![0u32]);
}
