use task_150::*;

#[test]
fn public_examples() {
    assert_eq!(fractional_fee_8(4, 0), 0);
    assert_eq!(fractional_fee_8(65535, 0), 5242);
}
