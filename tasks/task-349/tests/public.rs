use task_349::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_hash(String::from("outside[red#blue]ignored")),
        "blue"
    );
    assert_eq!(closing_field_hash(String::from("[猫###a]")), "a");
}
