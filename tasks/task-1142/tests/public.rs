use task_1142::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_hash(&"outside[red#blue]ignored"), 4);
    assert_eq!(closing_field_hash(&String::from("[猫###a]")), 1);
}
