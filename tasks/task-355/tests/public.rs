use task_355::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_hash(String::from("outside[red#blue]ignored")),
        "red"
    );
    assert_eq!(first_nonempty_field_hash(String::from("[猫###a]")), "猫");
}
