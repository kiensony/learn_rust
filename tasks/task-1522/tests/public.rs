use task_1522::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_total(&[]), 0);
    assert_eq!(packet_parity_total(&[4]), 1);
}
