use task_691::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("72;12")),
        Some(0)
    );
    assert_eq!(
        prefix_before_final_mark_equals(Event::Data("猫=12;note")),
        Some(3)
    );
}
