use task_348::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash(String::from("outside[red#blue]ignored")),
        "red"
    );
    assert_eq!(leading_field_hash(String::from("[猫###a]")), "猫");
}
