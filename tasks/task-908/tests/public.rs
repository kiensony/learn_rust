use task_908::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_colon("hi:kit", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(remainder_after_first_mark_colon(":é:a", |s| s.len() * 2), 8);
}
