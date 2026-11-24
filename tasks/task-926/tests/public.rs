use task_926::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_pipe("first\n123", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        closing_marks_removed_pipe("header\n猫|123", |s| s.len() * 2),
        14
    );
}
