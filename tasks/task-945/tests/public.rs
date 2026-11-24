use task_945::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        9
    );
    assert_eq!(leading_marks_removed_hash("[猫###a]", |s| s.len() * 2), 14);
}
