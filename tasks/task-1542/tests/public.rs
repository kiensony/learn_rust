use task_1542::*;

#[test]
fn public_examples() {
    assert_eq!(packet_reversal_total(&[]), 0);
    assert_eq!(packet_reversal_total(&[4]), 32);
}
