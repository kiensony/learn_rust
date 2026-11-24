use task_1016::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_slash(["path/1/12", "path/123"]),
        [4, 3]
    );
    assert_eq!(
        marks_removed_at_both_ends_slash([String::from("path/123"), String::from("hi/kit")]),
        [3, 6]
    );
}
