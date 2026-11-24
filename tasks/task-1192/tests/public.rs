use task_1192::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_hash("outside[red#blue]ignored", "backup"),
        "blue"
    );
    assert_eq!(closing_field_hash("[猫###a]", "backup"), "a");
}
