use task_1526::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_descending_steps(&[]), 0);
    assert_eq!(packet_parity_descending_steps(&[4]), 0);
}
