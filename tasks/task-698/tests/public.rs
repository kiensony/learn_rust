use task_698::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash(Event::Data("outside[red#blue]ignored")),
        Some(8)
    );
    assert_eq!(closing_marks_removed_hash(Event::Data("[猫###a]")), Some(7));
}
