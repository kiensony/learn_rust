use task_1028::*;

#[test]
fn public_examples() {
    assert_eq!(slots(&[1, 2]), 2);
    assert_eq!(slots(&[4]), 1);
}
