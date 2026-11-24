use task_947::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        9
    );
    assert_eq!(
        marks_removed_at_both_ends_hash("[猫###a]", |s| s.len() * 2),
        14
    );
}
