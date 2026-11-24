use task_208::*;

#[test]
fn public_examples() {
    assert_eq!(priority_fee_10(65535, 4), 10);
    assert_eq!(priority_fee_10(13, 14), 20);
}
