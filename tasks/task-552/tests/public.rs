use task_552::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash("outside[red#blue]ignored"),
        8
    );
    assert_eq!(marks_removed_at_both_ends_hash("[猫###a]"), 5);
}
