use task_120::*;

#[test]
fn public_examples() {
    assert_eq!(fractional_fee_2(4), 0);
    assert_eq!(fractional_fee_2(65535), 1310);
}
