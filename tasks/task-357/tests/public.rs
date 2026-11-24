use task_357::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(String::from("outside[red#blue]ignored")),
        "red"
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(String::from("[猫###a]")),
        ""
    );
}
