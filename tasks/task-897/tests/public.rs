use task_897::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash("outside[red#blue]ignored"),
        10
    );
    assert_eq!(marks_removed_at_both_ends_hash("[猫###a]"), 9);
}
