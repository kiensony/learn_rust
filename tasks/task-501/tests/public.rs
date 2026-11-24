use task_501::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash(("outside[red#blue]ignored", "x")),
        8
    );
    assert_eq!(closing_marks_removed_hash(("[猫###a]", "z#w")), 7);
}
