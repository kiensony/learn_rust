use task_1527::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_distinct(&[]), vec![]);
    assert_eq!(packet_parity_distinct(&[4]), vec![1u32]);
}
