use task_943::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(leading_field_hash("[猫###a]", |s| s.len() * 2), 6);
}
