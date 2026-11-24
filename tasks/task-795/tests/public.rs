use task_795::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash(&["outside[red#blue]ignored", "[猫###a]"]),
        std::collections::BTreeMap::from([
            (String::from("red#blue"), 1usize),
            (String::from("猫###a"), 1usize)
        ])
    );
    assert_eq!(
        leading_marks_removed_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"]),
        std::collections::BTreeMap::from([
            (String::from("red#blue"), 1usize),
            (String::from("猫###a"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
