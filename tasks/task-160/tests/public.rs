use task_160::*;

#[test]
fn public_examples() {
    assert_eq!(fractional_fee_10(65535, 4), 0);
    assert_eq!(fractional_fee_10(13, 14), 1);
}
