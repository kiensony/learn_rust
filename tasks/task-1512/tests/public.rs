use task_1512::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_total(&[]), 0);
    assert_eq!(packet_low_nibble_total(&[4]), 4);
}
