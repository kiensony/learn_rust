use task_1528::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_steps(&[]), vec![]);
    assert_eq!(packet_parity_steps(&[4]), vec![]);
}
