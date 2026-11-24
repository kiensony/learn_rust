use task_910::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_colon("hi:kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(first_nonempty_field_colon(":é:a", |s| s.len() * 2), 4);
}
