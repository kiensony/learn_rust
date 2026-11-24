use task_801::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(&["outside[red#blue]ignored", "[猫###a]"]),
        std::collections::BTreeMap::from([
            (String::from("blue"), 1usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_hash(&[
            "outside[red#blue]ignored",
            "[猫###a]",
            "unclosed[cat",
            "[]"
        ]),
        std::collections::BTreeMap::from([
            (String::from("blue"), 1usize),
            (String::from("猫"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
