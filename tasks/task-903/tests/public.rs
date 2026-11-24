use task_903::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon("hi:kit", |s| s.chars().count() + 1), 3);
    assert_eq!(leading_field_colon(":é:a", |s| s.len() * 2), 0);
}
