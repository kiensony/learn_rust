use task_1529::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_longest_run(&[]), 0);
    assert_eq!(packet_parity_longest_run(&[4]), 1);
}
