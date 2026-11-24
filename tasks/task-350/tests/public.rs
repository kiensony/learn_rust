use task_350::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash(String::from("outside[red#blue]ignored")),
        "red#blue"
    );
    assert_eq!(
        leading_marks_removed_hash(String::from("[猫###a]")),
        "猫###a"
    );
}
