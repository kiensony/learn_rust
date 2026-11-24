use task_1150::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(&"outside[red#blue]ignored"),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(&String::from("[猫###a]")),
        0
    );
}
