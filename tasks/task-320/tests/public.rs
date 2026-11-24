use task_320::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_slash(String::from("path/1/12")),
        "1/12"
    );
    assert_eq!(leading_marks_removed_slash(String::from("path/123")), "123");
}
