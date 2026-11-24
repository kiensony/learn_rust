use task_471::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_slash(("path/1/12", "x")), 4);
    assert_eq!(closing_marks_removed_slash(("path/123", "z/w")), 3);
}
