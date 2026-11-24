use task_911::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_colon("hi:kit", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        longest_field_with_last_tie_colon("", |s| s.chars().count() + 1),
        1
    );
}
