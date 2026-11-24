use task_918::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_slash("path/1/12", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        remainder_after_first_mark_slash("path/123", |s| s.len() * 2),
        0
    );
}
