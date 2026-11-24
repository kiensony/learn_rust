use task_797::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(&["outside[red#blue]ignored", "[猫###a]"]),
        std::collections::BTreeMap::from([
            (String::from("red#blue"), 1usize),
            (String::from("猫###a"), 1usize)
        ])
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(&[
            "outside[red#blue]ignored",
            "[猫###a]",
            "unclosed[cat",
            "[]"
        ]),
        std::collections::BTreeMap::from([
            (String::from("red#blue"), 1usize),
            (String::from("猫###a"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
