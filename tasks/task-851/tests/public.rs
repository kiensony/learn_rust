use task_851::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash("outside[red#blue]ignored"),
        Ok(4)
    );
    assert_eq!(longest_field_with_last_tie_hash("[猫###a]"), Ok(3));
}
