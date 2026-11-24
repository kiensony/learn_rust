use task_502::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(("outside[red#blue]ignored", "x")),
        8
    );
    assert_eq!(marks_removed_at_both_ends_hash(("[猫###a]", "z#w")), 7);
}
