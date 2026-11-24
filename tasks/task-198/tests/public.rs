use task_198::*;

#[test]
fn public_examples() {
    assert_eq!(priority_fee_8(4, 0), 8);
    assert_eq!(priority_fee_8(13, 1), 16);
}
