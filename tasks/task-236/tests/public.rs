use task_236::*;

#[test]
fn public_examples() {
    assert_eq!(open_count(&[true, false, true]), 2);
    assert_eq!(open_count(&[false, true]), 1);
}
