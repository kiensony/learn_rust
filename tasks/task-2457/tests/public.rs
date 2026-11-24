use task_2457::*;

#[test]
fn public_examples() {
    assert_eq!(checked_copy(&[1, 2, 3], 1, 1), Some(vec![2]));
    assert_eq!(checked_copy(&[1, 2, 3], 1, 2), Some(vec![2, 3]));
}
