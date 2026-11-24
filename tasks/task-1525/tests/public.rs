use task_1525::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_changes(&[]), vec![]);
    assert_eq!(packet_parity_changes(&[4]), vec![1u32]);
}
