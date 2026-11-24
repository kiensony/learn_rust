use task_1516::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_descending_steps(&[]), 0);
    assert_eq!(packet_low_nibble_descending_steps(&[4]), 0);
}
