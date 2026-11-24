use task_506::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(("outside[red#blue]ignored", "x")),
        4
    );
    assert_eq!(longest_field_with_last_tie_hash(("[猫###a]", "z#w")), 3);
}
