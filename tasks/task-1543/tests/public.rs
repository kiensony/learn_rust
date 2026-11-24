use task_1543::*;

#[test]
fn public_examples() {
    assert_eq!(packet_reversal_bounds(&[]), None);
    assert_eq!(packet_reversal_bounds(&[4]), Some((32u32, 32u32)));
}
