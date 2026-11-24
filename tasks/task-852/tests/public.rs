use task_852::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash("outside[red#blue]ignored"),
        Ok(3)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("[猫###a]"),
        Err(FieldError::Missing)
    );
}
