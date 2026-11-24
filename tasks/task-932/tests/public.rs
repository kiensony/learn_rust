use task_932::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_pipe("first\n123", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe("header\n猫|123", |s| s.len() * 2),
        6
    );
}
