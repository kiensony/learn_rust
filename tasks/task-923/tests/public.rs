use task_923::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_pipe("first\n123", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(leading_field_pipe("header\n猫|123", |s| s.len() * 2), 6);
}
