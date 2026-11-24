use task_602::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash("outside[red#blue]ignored"),
        "blue"
    );
    assert_eq!(remainder_after_first_mark_hash("[猫###a]"), "##a");
}
