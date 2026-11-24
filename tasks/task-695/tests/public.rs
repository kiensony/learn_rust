use task_695::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash(Event::Data("outside[red#blue]ignored")),
        Some(3)
    );
    assert_eq!(leading_field_hash(Event::Data("unclosed[cat")), Some(0));
}
