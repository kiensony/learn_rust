use task_606::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash("outside[red#blue]ignored"),
        "red"
    );
    assert_eq!(shortest_field_with_first_tie_hash("[猫###a]"), "");
}
