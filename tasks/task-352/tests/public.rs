use task_352::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(String::from("outside[red#blue]ignored")),
        "red#blue"
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(String::from("[猫###a]")),
        "猫###a"
    );
}
