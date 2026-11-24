use task_938::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_equals("72;12", |s| s.chars().count() + 1),
        1
    );
    assert_eq!(
        remainder_after_first_mark_equals("猫=12;note", |s| s.len() * 2),
        4
    );
}
