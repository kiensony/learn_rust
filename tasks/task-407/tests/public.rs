use task_407::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(&String::from("outside[red#blue]ignored")),
        3
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(&String::from("[猫###a]")),
        0
    );
}
