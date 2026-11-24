use task_1051::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(["outside[red#blue]ignored", "[猫###a]"]),
        [3, 0]
    );
    assert_eq!(
        shortest_field_with_first_tie_hash([
            String::from("[猫###a]"),
            String::from("unclosed[cat")
        ]),
        [0, 0]
    );
}
