use task_1507::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_distinct(&[]), vec![]);
    assert_eq!(packet_high_nibble_distinct(&[4]), vec![0u32]);
}
