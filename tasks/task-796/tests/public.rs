use task_796::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash(&["outside[red#blue]ignored", "[猫###a]"]),
        std::collections::BTreeMap::from([
            (String::from("red#blue"), 1usize),
            (String::from("猫###a"), 1usize)
        ])
    );
    assert_eq!(
        closing_marks_removed_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"]),
        std::collections::BTreeMap::from([
            (String::from("red#blue"), 1usize),
            (String::from("猫###a"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
