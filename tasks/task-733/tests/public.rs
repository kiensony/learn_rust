use task_733::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(Some("first\n123")),
        Some(123)
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(Some("header\n猫|123")),
        None
    );
}
