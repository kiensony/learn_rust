use task_1020::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash(["path/1/12", "path/123"]),
        [2, 3]
    );
    assert_eq!(
        longest_field_with_last_tie_slash([String::from("path/123"), String::from("hi/kit")]),
        [3, 3]
    );
}
