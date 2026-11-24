use task_1541::*;

#[test]
fn public_examples() {
    assert_eq!(packet_reversal_trace(&[]), vec![]);
    assert_eq!(packet_reversal_trace(&[4]), vec![32u32]);
}
