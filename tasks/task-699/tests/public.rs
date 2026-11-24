use task_699::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(Event::Data("outside[red#blue]ignored")),
        Some(8)
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(Event::Data("[猫###a]")),
        Some(7)
    );
}
