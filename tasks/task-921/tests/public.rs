use task_921::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash("path/1/12", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        longest_field_with_last_tie_slash("path/123", |s| s.len() * 2),
        6
    );
}
