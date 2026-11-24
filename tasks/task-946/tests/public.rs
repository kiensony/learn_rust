use task_946::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        9
    );
    assert_eq!(closing_marks_removed_hash("[猫###a]", |s| s.len() * 2), 14);
}
