use task_763::*;

#[test]
fn public_examples() {
    assert_eq!(served(&[2, 1, 2], 2), vec![2, 1]);
    assert_eq!(served(&[4, 5], 3), vec![4, 5]);
}
