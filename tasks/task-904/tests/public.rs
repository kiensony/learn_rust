use task_904::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon("hi:kit", |s| s.chars().count() + 1), 4);
    assert_eq!(closing_field_colon(":é:a", |s| s.len() * 2), 2);
}
