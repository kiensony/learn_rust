use task_914::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_slash("path/1/12", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(closing_field_slash("path/123", |s| s.len() * 2), 6);
}
