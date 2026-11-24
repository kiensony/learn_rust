use task_1515::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_changes(&[]), vec![]);
    assert_eq!(packet_low_nibble_changes(&[4]), vec![4u32]);
}
