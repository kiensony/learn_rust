use task_1196::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash("outside[red#blue]ignored", "backup"),
        "blue"
    );
    assert_eq!(remainder_after_first_mark_hash("[猫###a]", "backup"), "##a");
}
