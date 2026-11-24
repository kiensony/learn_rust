use task_714::*;

#[test]
fn public_examples() {
    assert_eq!(difference(Some(2), Some(8)), Some(6));
    assert_eq!(difference(Some(3), Some(-4)), Some(-7));
}
