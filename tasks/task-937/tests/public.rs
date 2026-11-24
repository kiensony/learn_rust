use task_937::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_equals("72;12", |s| s.chars().count() + 1),
        3
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("猫=12;note", |s| s.len() * 2),
        12
    );
}
