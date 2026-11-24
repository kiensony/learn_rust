use task_1200::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash("outside[red#blue]ignored", "backup"),
        "red"
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("[猫###a]", "backup"),
        "backup"
    );
}
