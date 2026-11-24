use task_1519::*;

#[test]
fn public_examples() {
    assert_eq!(packet_low_nibble_longest_run(&[]), 0);
    assert_eq!(packet_low_nibble_longest_run(&[4]), 1);
}
