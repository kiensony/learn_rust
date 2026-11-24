use task_500::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash(("outside[red#blue]ignored", "x")),
        8
    );
    assert_eq!(leading_marks_removed_hash(("[猫###a]", "z#w")), 7);
}
