use task_1518::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_steps(&[]), vec![]);
    assert_eq!(packet_low_nibble_steps(&[4]), vec![]);
}
