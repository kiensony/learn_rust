use task_696::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_hash(Event::Data("outside[red#blue]ignored")),
        Some(4)
    );
    assert_eq!(closing_field_hash(Event::Data("[猫###a]")), Some(1));
}
