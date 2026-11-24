use task_847::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash("outside[red#blue]ignored"),
        Err(FieldError::TooLong)
    );
    assert_eq!(marks_removed_at_both_ends_hash("[猫###a]"), Ok(7));
}
