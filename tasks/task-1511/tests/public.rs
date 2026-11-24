use task_1511::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_trace(&[]), vec![]);
    assert_eq!(packet_low_nibble_trace(&[4]), vec![4u32]);
}
