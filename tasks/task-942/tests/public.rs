use task_942::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals("72;12", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_equals("猫=12;note", |s| s.len() * 2),
        4
    );
}
