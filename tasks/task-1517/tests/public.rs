use task_1517::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_distinct(&[]), vec![]);
    assert_eq!(packet_low_nibble_distinct(&[4]), vec![4u32]);
}
