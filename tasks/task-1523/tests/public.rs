use task_1523::*;

#[test]
fn public_examples() {
    assert_eq!(packet_parity_bounds(&[]), None);
    assert_eq!(packet_parity_bounds(&[4]), Some((1u32, 1u32)));
}
