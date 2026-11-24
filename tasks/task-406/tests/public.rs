use task_406::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(&String::from("outside[red#blue]ignored")),
        4
    );
    assert_eq!(
        longest_field_with_last_tie_hash(&String::from("[猫###a]")),
        3
    );
}
