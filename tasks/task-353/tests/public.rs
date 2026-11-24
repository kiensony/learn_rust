use task_353::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(String::from("outside[red#blue]ignored")),
        "blue"
    );
    assert_eq!(
        remainder_after_first_mark_hash(String::from("[猫###a]")),
        "##a"
    );
}
