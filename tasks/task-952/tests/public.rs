use task_952::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash("outside[red#blue]ignored", |s| s.chars().count() + 1),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_hash("[猫###a]", |s| s.len() * 2),
        0
    );
}
