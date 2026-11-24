use task_1530::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_reverse(&[]), vec![]);
    assert_eq!(packet_parity_reverse(&[4]), vec![1u32]);
}
