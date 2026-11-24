use task_909::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_colon("hi:kit", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(prefix_before_final_mark_colon(":é:a", |s| s.len() * 2), 6);
}
