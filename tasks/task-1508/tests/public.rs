use task_1508::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_steps(&[]), vec![]);
    assert_eq!(packet_high_nibble_steps(&[4]), vec![]);
}
