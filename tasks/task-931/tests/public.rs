use task_931::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe("first\n123", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        longest_field_with_last_tie_pipe("header\n猫|123", |s| s.len() * 2),
        6
    );
}
