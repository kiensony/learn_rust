use task_951::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        5
    );
    assert_eq!(
        longest_field_with_last_tie_hash("[猫###a]", |s| s.len() * 2),
        6
    );
}
