use task_356::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(String::from("outside[red#blue]ignored")),
        "blue"
    );
    assert_eq!(
        longest_field_with_last_tie_hash(String::from("[猫###a]")),
        "猫"
    );
}
