use task_468::*;

#[test]
fn public_examples() {
    assert_eq!(shift([1, 2, 3]), [2, 3, 1]);
    assert_eq!(shift([9, 0, 2]), [0, 2, 9]);
}
