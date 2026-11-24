use task_1015::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_slash(["path/1/12", "path/123"]),
        [4, 3]
    );
    assert_eq!(
        closing_marks_removed_slash([String::from("path/123"), String::from("hi/kit")]),
        [3, 6]
    );
}
