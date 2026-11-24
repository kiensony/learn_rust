use task_802::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(&["outside[red#blue]ignored", "[猫###a]"]),
        std::collections::BTreeMap::from([
            (String::from("red"), 1usize),
            (String::from(""), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(&[
            "outside[red#blue]ignored",
            "[猫###a]",
            "unclosed[cat",
            "[]"
        ]),
        std::collections::BTreeMap::from([
            (String::from("red"), 1usize),
            (String::from(""), 3usize)
        ])
    );
}
