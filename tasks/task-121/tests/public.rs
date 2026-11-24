use task_121::*;

#[test]
fn public_examples() {
    assert_eq!(as_byte(42), Some(42));
    assert_eq!(as_byte(255), Some(255));
}
