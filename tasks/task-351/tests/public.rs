use task_351::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash(String::from("outside[red#blue]ignored")),
        "red#blue"
    );
    assert_eq!(
        closing_marks_removed_hash(String::from("[猫###a]")),
        "猫###a"
    );
}
