use task_178::*;

#[test]
fn public_examples() {
    assert_eq!(priority_fee_3(4, 0), 3);
    assert_eq!(priority_fee_3(13, 3), 6);
}
