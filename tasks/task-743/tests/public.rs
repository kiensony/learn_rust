use task_743::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals(Some("72;12")),
        Some(72)
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(Some("猫=12;note")),
        Some(12)
    );
}
