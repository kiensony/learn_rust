use task_703::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(Event::Data("outside[red#blue]ignored")),
        Some(4)
    );
    assert_eq!(
        longest_field_with_last_tie_hash(Event::Data("[猫###a]")),
        Some(3)
    );
}
