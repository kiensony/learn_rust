use task_130::*;

#[test]
fn public_examples() {
    assert_eq!(fractional_fee_3(4, 0), 0);
    assert_eq!(fractional_fee_3(65535, 4), 1966);
}
