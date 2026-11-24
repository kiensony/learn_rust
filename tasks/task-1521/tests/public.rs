use task_1521::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_trace(&[]), vec![]);
    assert_eq!(packet_parity_trace(&[4]), vec![1u32]);
}
