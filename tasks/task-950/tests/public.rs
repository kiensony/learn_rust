use task_950::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(first_nonempty_field_hash("[猫###a]", |s| s.len() * 2), 6);
}
