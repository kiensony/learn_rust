use task_902::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash("outside[red#blue]ignored"),
        5
    );
    assert_eq!(shortest_field_with_first_tie_hash("[猫###a]"), 2);
}
