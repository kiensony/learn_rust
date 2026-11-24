use task_140::*;

#[test]
fn public_examples() {
    assert_eq!(fractional_fee_5(4, 0), 0);
    assert_eq!(fractional_fee_5(65535, 1), 3276);
}
