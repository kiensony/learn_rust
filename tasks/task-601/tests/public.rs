use task_601::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash("outside[red#blue]ignored"),
        "red#blue"
    );
    assert_eq!(marks_removed_at_both_ends_hash("[猫###a]"), "猫###a");
}
