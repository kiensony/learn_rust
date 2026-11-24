use task_1501::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_trace(&[]), vec![]);
    assert_eq!(packet_high_nibble_trace(&[4]), vec![0u32]);
}
