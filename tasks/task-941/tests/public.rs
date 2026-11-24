use task_941::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_equals("72;12", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        longest_field_with_last_tie_equals("猫=12;note", |s| s.len() * 2),
        6
    );
}
