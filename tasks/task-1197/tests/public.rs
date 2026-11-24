use task_1197::*;

#[test]
fn public_examples() {
    assert_eq!(columns_of(&["a:b", "c:d"]), vec![("a", "b"), ("c", "d")]);
    assert_eq!(columns_of(&["none", "x:y:z"]), vec![("x", "y:z")]);
}
