use task_402::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(&String::from("outside[red#blue]ignored")),
        8
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(&String::from("[猫###a]")),
        7
    );
}
