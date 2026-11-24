use task_1545::*;

#[test]
fn public_examples() {
    assert_eq!(packet_reversal_changes(&[]), vec![]);
    assert_eq!(packet_reversal_changes(&[4]), vec![32u32]);
}
