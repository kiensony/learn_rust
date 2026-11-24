use task_1096::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(&"outside[red#blue]ignored"),
        4
    );
    assert_eq!(
        remainder_after_first_mark_hash(&String::from("[猫###a]")),
        3
    );
}
