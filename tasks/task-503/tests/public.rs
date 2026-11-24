use task_503::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(("outside[red#blue]ignored", "x")),
        4
    );
    assert_eq!(remainder_after_first_mark_hash(("[猫###a]", "z#w")), 3);
}
