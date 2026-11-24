use task_800::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_hash(&["outside[red#blue]ignored", "[猫###a]"]),
        std::collections::BTreeMap::from([
            (String::from("red"), 1usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        first_nonempty_field_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"]),
        std::collections::BTreeMap::from([
            (String::from("red"), 1usize),
            (String::from("猫"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
