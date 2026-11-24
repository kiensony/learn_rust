use task_928::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe("first\n123", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        remainder_after_first_mark_pipe("header\n猫|123", |s| s.len() * 2),
        6
    );
}
