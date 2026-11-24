use task_157::*;

#[test]
fn public_examples() {
    assert_eq!(milliseconds(2), Some(2000));
    assert_eq!(milliseconds(-1), None);
}
