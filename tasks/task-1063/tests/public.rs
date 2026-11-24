use task_1063::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_slash(&"path/1/12"), 4);
    assert_eq!(leading_marks_removed_slash(&String::from("path/123")), 3);
}
