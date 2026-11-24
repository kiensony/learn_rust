use task_700::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(Event::Data("outside[red#blue]ignored")),
        Some(4)
    );
    assert_eq!(
        remainder_after_first_mark_hash(Event::Data("[猫###a]")),
        Some(3)
    );
}
