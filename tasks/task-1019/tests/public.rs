use task_1019::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_slash(["path/1/12", "path/123"]),
        [1, 3]
    );
    assert_eq!(
        first_nonempty_field_slash([String::from("path/123"), String::from("hi/kit")]),
        [3, 2]
    );
}
