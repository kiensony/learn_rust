use task_704::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("outside[red#blue]ignored")),
        Some(3)
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(Event::Data("[猫###a]")),
        Some(0)
    );
}
