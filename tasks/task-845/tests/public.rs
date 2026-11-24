use task_845::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash("outside[red#blue]ignored"),
        Err(FieldError::TooLong)
    );
    assert_eq!(leading_marks_removed_hash("[猫###a]"), Ok(7));
}
