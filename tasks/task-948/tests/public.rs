use task_948::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        5
    );
    assert_eq!(
        remainder_after_first_mark_hash("[猫###a]", |s| s.len() * 2),
        6
    );
}
