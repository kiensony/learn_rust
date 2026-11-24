use task_927::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_pipe("first\n123", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\n猫|123", |s| s.len() * 2),
        14
    );
}
