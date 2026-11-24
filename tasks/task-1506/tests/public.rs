use task_1506::*;

#[test]
fn public_examples() {
    assert_eq!(packet_high_nibble_descending_steps(&[]), 0);
    assert_eq!(packet_high_nibble_descending_steps(&[4]), 0);
}
