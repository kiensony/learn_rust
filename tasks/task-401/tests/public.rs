use task_401::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash(&String::from("outside[red#blue]ignored")),
        8
    );
    assert_eq!(closing_marks_removed_hash(&String::from("[猫###a]")), 7);
}
