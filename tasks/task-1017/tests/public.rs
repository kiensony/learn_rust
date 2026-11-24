use task_1017::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_slash(["path/1/12", "path/123"]),
        [2, 0]
    );
    assert_eq!(
        remainder_after_first_mark_slash([String::from("path/123"), String::from("hi/kit")]),
        [0, 3]
    );
}
