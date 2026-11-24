use task_690::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_equals(Event::Data("72;12")),
        Some(0)
    );
    assert_eq!(
        remainder_after_first_mark_equals(Event::Data("猫=12;note")),
        Some(2)
    );
}
