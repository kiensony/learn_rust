use task_944::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        5
    );
    assert_eq!(closing_field_hash("[猫###a]", |s| s.len() * 2), 2);
}
