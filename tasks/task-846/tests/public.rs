use task_846::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash("outside[red#blue]ignored"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_hash("[猫###a]"), Ok(7));
}
