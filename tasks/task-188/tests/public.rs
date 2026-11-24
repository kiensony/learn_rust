use task_188::*;

#[test]
fn public_examples() {
    assert_eq!(priority_fee_5(4, 0), 5);
    assert_eq!(priority_fee_5(13, 1), 10);
}
