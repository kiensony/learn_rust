use task_934::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals("72;12", |s| s.chars().count() + 1), 3);
    assert_eq!(closing_field_equals("猫=12;note", |s| s.len() * 2), 4);
}
